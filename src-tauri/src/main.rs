mod models;
mod ai_client;
mod prompts;
mod game_logic;

use std::sync::Arc;
use tokio::sync::Mutex;
use tauri::{State, Manager};
use models::{Card, AttackJudgement, GameState, AppError};
use game_logic::GameLogic;
use ai_client::GeminiClient;

/// アプリケーション全体の共有状態を管理する構造体
struct AppState {
    /// 進行中のゲームインスタンス（非同期Mutexで管理）
    game: Mutex<Option<GameLogic>>,
    /// AIクライアントの設定（不変な状態で保持）
    api_key: String,
}

/// ゲームを開始するTauriコマンド
#[tauri::command]
async fn start_game(
    state: State<'_, AppState>, 
    handle: tauri::AppHandle,
    theme: Option<String>
) -> Result<GameState, AppError> {
    // リソースパスの取得（Tauriのパスレゾルバを使用）
    let prompts_path = handle.path_resolver()
        .resolve_resource("prompts.yaml")
        .ok_or_else(|| AppError::ConfigError("prompts.yaml not found".to_string()))?;
    
    // AIクライアントの初期化（APIキーはAppStateから取得）
    let ai_client = GeminiClient::new(
        state.api_key.clone(), 
        "gemini-2.0-flash".to_string(), 
        &prompts_path
    );
    
    // ゲームロジックの初期化と開始
    let mut game = GameLogic::new(ai_client);
    let initial_state = game.start_new_game(theme);
    
    // 状態をグローバルに保存
    let mut app_game = state.game.lock().await;
    *app_game = Some(game);
    
    Ok(initial_state)
}

/// 攻撃を実行するTauriコマンド
#[tauri::command]
async fn play_attack(
    state: State<'_, AppState>, 
    claim: Card, 
    reason: Card, 
    fact: Card
) -> Result<(AttackJudgement, bool), AppError> {
    let mut app_game = state.game.lock().await;
    if let Some(game) = app_game.as_mut() {
        // Rust側のゲームロジックで攻撃を処理（await中もMutexが保持されるが、tokioのMutexなのでデッドロックを回避可能）
        game.play_attack_turn(claim, reason, fact, true).await
    } else {
        Err(AppError::GameError("ゲームが開始されていません".to_string()))
    }
}

fn main() {
    // .env ファイルの読み込み
    dotenv::dotenv().ok();
    let api_key = std::env::var("GEMINI_API_KEY")
        .unwrap_or_else(|_| "YOUR_API_KEY_HERE".to_string());

    tauri::Builder::default()
        // アプリケーション状態の登録
        .manage(AppState {
            game: Mutex::new(None),
            api_key,
        })
        // フロントエンドから呼び出し可能なコマンドの登録
        .invoke_handler(tauri::generate_handler![start_game, play_attack])
        .run(tauri::generate_context!())
        .expect("Tauriアプリケーションの実行中にエラーが発生しました");
}
