use crate::ai_client::GeminiClient;
use crate::models::{Card, GameState, AttackJudgement, DefenseJudgement};
use rand::seq::SliceRandom;

/// ゲームのターンフェーズ
pub enum TurnPhase {
    /// 攻撃フェーズ
    Attack,
    /// 防御フェーズ
    Defense,
}

/// ゲームの核となるロジックとルールを管理する構造体
pub struct GameLogic {
    /// AIクライアント
    ai_client: GeminiClient,
    /// 現在のゲーム状態
    state: Option<GameState>,
    /// 現在のフェーズ
    current_phase: TurnPhase,
    /// 直前の敵の攻撃内容（防御フェーズで使用）
    last_enemy_attack: Option<String>,
}

impl GameLogic {
    /// ランダム選択用のサンプルテーマ
    const SAMPLE_THEMES: [(&'static str, &'static str, &'static str); 5] = [
        ("きのこの山 vs たけのこの里", "きのこの山派", "たけのこの里派"),
        ("犬 vs 猫", "犬派", "猫派"),
        ("朝型 vs 夜型", "朝型派", "夜型派"),
        ("海 vs 山", "海派", "山派"),
        ("カレー vs ラーメン", "カレー派", "ラーメン派"),
    ];

    /// 新しいGameLogicインスタンスを作成する
    pub fn new(ai_client: GeminiClient) -> Self {
        Self {
            ai_client,
            state: None,
            current_phase: TurnPhase::Attack,
            last_enemy_attack: None,
        }
    }

    /// 新しいゲームを開始し、初期状態を返す
    /// themeが指定されない場合は、サンプルからランダムに選択する
    pub fn start_new_game(&mut self, theme: Option<String>) -> GameState {
        let (selected_theme, player_pos, opponent_pos) = if let Some(t) = theme {
            (t.clone(), format!("{}賛成派", t), format!("{}反対派", t))
        } else {
            let mut rng = rand::thread_rng();
            let sample = Self::SAMPLE_THEMES.choose(&mut rng).unwrap();
            (sample.0.to_string(), sample.1.to_string(), sample.2.to_string())
        };

        let initial_state = GameState {
            theme: selected_theme,
            player_position: player_pos,
            opponent_position: opponent_pos,
            gauge: 0,
            history: Vec::new(),
            turn_count: 1,
        };

        self.state = Some(initial_state.clone());
        self.current_phase = TurnPhase::Attack;
        self.last_enemy_attack = None;

        initial_state
    }

    /// 攻撃ターンを実行する
    /// AIによる判定を行い、ゲージを更新する
    pub async fn play_attack_turn(&mut self, claim: Card, reason: Card, fact: Card, is_player_turn: bool) -> Result<(AttackJudgement, bool), String> {
        let state = self.state.as_mut().ok_or("Game not started")?;
        
        // AIによる攻撃の判定
        let result = self.ai_client.judge_attack(&state.theme, &state.history, &claim.text, &reason.text, &fact.text).await?;
        
        // スコアに基づいてゲージを移動（ゲームを長引かせるために0.5倍にスケール）
        let move_val = (result.total_score as f32 * 0.5) as i32;
        if is_player_turn {
            state.gauge += move_val;
        } else {
            state.gauge -= move_val;
        }
        
        // ゲージを-100から100の範囲に収める
        state.gauge = state.gauge.clamp(-100, 100);
        
        // 履歴を更新
        let history_entry = if is_player_turn {
            format!("(プレイヤー攻撃) {} / {} / {}", claim.text, reason.text, fact.text)
        } else {
            format!("(相手攻撃) {} / {} / {}", claim.text, reason.text, fact.text)
        };
        state.history.push(history_entry);
        
        // 次の防御フェーズのために攻撃内容を保存
        self.last_enemy_attack = Some(format!("Claim: {}\nReason: {}\nFact: {}", claim.text, reason.text, fact.text));
        state.turn_count += 1;
        
        // ゲーム終了判定（ゲージが端に達したか）
        let is_over = state.gauge.abs() >= 100;
        Ok((result, is_over))
    }
}
