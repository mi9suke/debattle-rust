mod models;
mod ai_client;
mod prompts;
mod game_logic;

use std::sync::Mutex;
use tauri::State;
use models::{Card, AttackJudgement, GameState};
use game_logic::GameLogic;
use ai_client::GeminiClient;
use std::path::PathBuf;

/// アプリケーション全体の共有状態を管理する構造体
struct AppState {
    /// 進行中のゲームインスタンス（スレッドセーフに管理）
    game: Mutex<Option<GameLogic>>,
}

/// ゲームを開始するTauriコマンド
/// theme: 指定された討論テーマ（Option）
#[tauri::command]
async fn start_game(state: State<'_, AppState>, theme: Option<String>) -> Result<GameState, String> {
    // TODO: .env または設定ファイルからAPIキーを読み込むように変更する
    let api_key = "YOUR_API_KEY".to_string(); 
    let prompts_path = PathBuf::from("prompts.yaml");
    
    // AIクライアントの初期化
    let ai_client = GeminiClient::new(api_key, "gemini-2.0-flash".to_string(), &prompts_path);
    
    // ゲームロジックの初期化と開始
    let mut game = GameLogic::new(ai_client);
    let initial_state = game.start_new_game(theme);
    
    // 状態をグローバルに保存
    let mut app_game = state.game.lock().unwrap();
    *app_game = Some(game);
    
    Ok(initial_state)
}

/// 攻撃を実行するTauriコマンド
#[tauri::command]
async fn play_attack(state: State<'_, AppState>, claim: Card, reason: Card, fact: Card) -> Result<(AttackJudgement, bool), String> {
    let mut app_game = state.game.lock().unwrap();
    if let Some(game) = app_game.as_mut() {
        // Rust側のゲームロジックで攻撃を処理
        game.play_attack_turn(claim, reason, fact, true).await
    } else {
        Err("ゲームが開始されていません".to_string())
    }
}

fn main() {
    tauri::Builder::default()
        // アプリケーション状態の登録
        .manage(AppState {
            game: Mutex::new(None),
        })
        // フロントエンドから呼び出し可能なコマンドの登録
        .invoke_handler(tauri::generate_handler![start_game, play_attack])
        .run(tauri::generate_context!())
        .expect("Tauriアプリケーションの実行中にエラーが発生しました");
}
