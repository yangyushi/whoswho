use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Person {
    pub id: i64,
    pub name: String,
    #[serde(flatten)]
    pub fields: HashMap<String, String>,
    pub created_at: DateTime<Local>,
    pub updated_at: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Note {
    pub id: i64,
    pub person_id: i64,
    pub content: String,
    pub created_at: DateTime<Local>,
}

#[derive(Debug, Default, Clone)]
pub struct PersonUpdate {
    pub name: Option<String>,
    pub fields: HashMap<String, String>,
}
