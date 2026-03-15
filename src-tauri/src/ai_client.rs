use crate::models::{AppError, Card, Hand, AttackJudgement, DefenseJudgement};
use crate::prompts::PromptManager;
use serde_json::{json, Value};
use std::path::Path;

/// Google Gemini API と通信するためのクライアント
pub struct GeminiClient {
    api_key: String,
    model_name: String,
    prompt_manager: PromptManager,
    client: reqwest::Client,
}

impl GeminiClient {
    pub fn new(api_key: String, model_name: String, prompts_path: &Path) -> Self {
        let prompt_manager = PromptManager::load(prompts_path);
        let client = reqwest::Client::new();

        Self {
            api_key,
            model_name,
            prompt_manager,
            client,
        }
    }

    /// Gemini API への共通リクエスト処理
    async fn call_gemini(&self, system_instruction: &str, user_prompt: &str) -> Result<String, AppError> {
        let url = format!(
            "https://generativelanguage.googleapis.com/v1beta/models/{}:generateContent?key={}",
            self.model_name, self.api_key
        );

        let body = json!({
            "system_instruction": {
                "parts": { "text": system_instruction }
            },
            "contents": {
                "parts": { "text": user_prompt }
            },
            "generationConfig": {
                "response_mime_type": "application/json",
                "temperature": 0.7,
            }
        });

        let response = self.client.post(&url)
            .json(&body)
            .send()
            .await?;

        if !response.status().is_success() {
            let error_text = response.text().await?;
            return Err(AppError::ApiError(format!("Gemini API error: {}", error_text)));
        }

        let json_body: Value = response.json().await?;
        
        // Gemini API のレスポンスからテキスト部分を抽出
        let text = json_body["candidates"][0]["content"]["parts"][0]["text"]
            .as_str()
            .ok_or_else(|| AppError::ApiError("Failed to extract text from Gemini response".to_string()))?;
        
        Ok(text.to_string())
    }

    /// 手札を生成する
    pub async fn generate_hand(&self, theme: &str, position: &str, incoming_attack: Option<&str>) -> Result<Hand, AppError> {
        let system = self.prompt_manager.get_system_instruction();
        let prompt = self.prompt_manager.format_generate_hand(theme, position, incoming_attack);
        
        let response_text = self.call_gemini(system, &prompt).await?;
        let hand: Hand = serde_json::from_str(&response_text)
            .map_err(|e| AppError::ApiError(format!("Failed to parse hand JSON: {}", e)))?;
            
        Ok(hand)
    }

    /// 攻撃を判定する
    pub async fn judge_attack(&self, theme: &str, history: &[String], claim: &str, reason: &str, fact: &str) -> Result<AttackJudgement, AppError> {
        let system = self.prompt_manager.get_system_instruction();
        let prompt = self.prompt_manager.format_judge_attack(theme, history, claim, reason, fact);
        
        let response_text = self.call_gemini(system, &prompt).await?;
        let result: AttackJudgement = serde_json::from_str(&response_text)
            .map_err(|e| AppError::ApiError(format!("Failed to parse judgement JSON: {}", e)))?;
            
        Ok(result)
    }

    /// 防御を判定する
    pub async fn judge_defense(&self, theme: &str, enemy_attack: &str, player_card: &Card, support_card: &Card) -> Result<DefenseJudgement, AppError> {
        let system = self.prompt_manager.get_system_instruction();
        let prompt = self.prompt_manager.format_judge_defense(theme, enemy_attack, player_card, support_card);
        
        let response_text = self.call_gemini(system, &prompt).await?;
        let result: DefenseJudgement = serde_json::from_str(&response_text)
            .map_err(|e| AppError::ApiError(format!("Failed to parse defense JSON: {}", e)))?;
            
        Ok(result)
    }
}
