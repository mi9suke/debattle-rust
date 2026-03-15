use crate::models::{Card, Hand, AttackJudgement, DefenseJudgement};
use std::path::Path;
use std::fs;

/// Google Gemini API と通信するためのクライアント
pub struct GeminiClient {
    api_key: String,
    model_name: String,
    prompts: serde_yaml::Value,
}

impl GeminiClient {
    pub fn new(api_key: String, model_name: String, prompts_path: &Path) -> Self {
        let prompts_str = fs::read_to_string(prompts_path).expect("Failed to read prompts file");
        let prompts: serde_yaml::Value = serde_yaml::from_str(&prompts_str).expect("Failed to parse YAML");

        Self {
            api_key,
            model_name,
            prompts,
        }
    }

    /// 手札を生成する
    pub async fn generate_hand(&self, theme: &str, position: &str, incoming_attack: Option<&str>) -> Result<Hand, String> {
        // TODO: Implement actual API call to Gemini
        Err("Not implemented yet".to_string())
    }

    /// 攻撃を判定する
    pub async fn judge_attack(&self, theme: &str, history: &[String], claim: &str, reason: &str, fact: &str) -> Result<AttackJudgement, String> {
        // TODO: Implement actual API call to Gemini
        Err("Not implemented yet".to_string())
    }

    /// 防御を判定する
    pub async fn judge_defense(&self, theme: &str, enemy_attack: &str, player_card: &Card, support_card: &Card) -> Result<DefenseJudgement, String> {
        // TODO: Implement actual API call to Gemini
        Err("Not implemented yet".to_string())
    }
}
