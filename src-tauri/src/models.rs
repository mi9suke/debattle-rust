use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use thiserror::Error;

/// アプリケーション固有のエラー型
#[derive(Debug, Error, Serialize)]
pub enum AppError {
    #[error("API error: {0}")]
    ApiError(String),
    #[error("Game error: {0}")]
    GameError(String),
    #[error("Config error: {0}")]
    ConfigError(String),
    #[error("IO error: {0}")]
    IoError(String),
}

impl From<reqwest::Error> for AppError {
    fn from(e: reqwest::Error) -> Self {
        AppError::ApiError(e.to_string())
    }
}

impl From<std::io::Error> for AppError {
    fn from(e: std::io::Error) -> Self {
        AppError::IoError(e.to_string())
    }
}

/// カードの種類
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum CardType {
    #[serde(rename = "Claim")]
    Claim,
    #[serde(rename = "Reason")]
    Reason,
    #[serde(rename = "Fact")]
    Fact,
    #[serde(rename = "Counter")]
    Counter,
}

/// 個別の討論カードを表す
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Card {
    pub id: i32,
    #[serde(rename = "type")]
    pub card_type: CardType,
    pub text: String,
}

/// 手札を表す
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Hand {
    pub cards: Vec<Card>,
}

/// 攻撃フェーズの判定結果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AttackJudgement {
    pub total_score: i32,
    pub breakdown: HashMap<String, i32>,
    pub mascot_comment: String,
}

/// 防御フェーズの判定結果
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DefenseJudgement {
    pub defense_success: bool,
    pub damage_taken: i32,
    pub reason: String,
    pub mascot_comment: String,
}

/// 現在のゲーム状態
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct GameState {
    pub theme: String,
    pub player_position: String,
    pub opponent_position: String,
    pub gauge: i32,
    pub history: Vec<String>,
    pub turn_count: i32,
}
