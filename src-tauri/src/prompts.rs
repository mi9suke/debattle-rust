use std::fs;
use std::path::Path;
use crate::models::Card;

/// プロンプト管理クラス
pub struct PromptManager {
    prompts: serde_yaml::Value,
}

impl PromptManager {
    pub fn load(path: &Path) -> Self {
        let content = fs::read_to_string(path).expect("Failed to read prompts.yaml");
        let prompts: serde_yaml::Value = serde_yaml::from_str(&content).expect("Failed to parse prompts.yaml");
        Self { prompts }
    }

    pub fn get_system_instruction(&self) -> &str {
        self.prompts["system"].as_str().unwrap_or("")
    }

    pub fn format_generate_hand(&self, theme: &str, position: &str, incoming_attack: Option<&str>) -> String {
        let template = self.prompts["generate_hand"].as_str().unwrap_or("");
        let attack_str = incoming_attack.map(|a| format!("Incoming Attack to Refute:\n{}", a)).unwrap_or_default();
        template.replace("{theme}", theme)
                .replace("{position}", position)
                .replace("{incoming_attack}", &attack_str)
    }

    pub fn format_judge_attack(&self, theme: &str, history: &[String], claim: &str, reason: &str, fact: &str) -> String {
        let template = self.prompts["judge_attack"].as_str().unwrap_or("");
        let history_str = if history.is_empty() { "なし" } else { &history.join("\n") };
        template.replace("{theme}", theme)
                .replace("{history}", history_str)
                .replace("{claim}", claim)
                .replace("{reason}", reason)
                .replace("{fact}", fact)
    }

    pub fn format_judge_defense(&self, theme: &str, enemy_attack: &str, player_card: &Card, support_card: &Card) -> String {
        let template = self.prompts["judge_defense"].as_str().unwrap_or("");
        template.replace("{theme}", theme)
                .replace("{enemy_attack}", enemy_attack)
                .replace("{player_card}", &player_card.text)
                .replace("{support_card}", &support_card.text)
    }
}
