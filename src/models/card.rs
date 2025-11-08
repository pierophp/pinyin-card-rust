use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Card {
    pub id: i32,
    pub name_en: Option<String>,
    pub name_pt: Option<String>,
    pub name_cht: Option<String>,
    pub name_chs: Option<String>,
    pub image: Option<String>,
    pub category_id: i32,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
    pub pinyin: Option<String>,
    pub audio_ch: Option<String>,
    pub audio_en: Option<String>,
    pub audio_pt: Option<String>,
    pub audio_fr: Option<String>,
    pub audio_it: Option<String>,
    pub name_fr: Option<String>,
    pub name_it: Option<String>,
    pub extra_ch: Option<Value>,
    pub extra_en: Option<Value>,
    pub extra_fr: Option<Value>,
    pub extra_it: Option<Value>,
    pub extra_pt: Option<Value>,
    pub audio_de: Option<String>,
    pub extra_de: Option<Value>,
    pub name_de: Option<String>,
}
