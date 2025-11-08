use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, FromRow)]
pub struct Category {
    pub id: i32,
    pub name_en: Option<String>,
    pub name_pt: Option<String>,
    pub name_cht: Option<String>,
    pub name_chs: Option<String>,
    pub name_fr: Option<String>,
    pub name_it: Option<String>,
    pub name_de: Option<String>,
    pub parent_category_id: Option<i32>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}
