use chrono::Utc;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowInfo {
    pub exec_name: String,
    pub name: String,
    pub path: String,
    pub process_id: u32,
    pub timestamp: Option<chrono::DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowPosition {
    pub x: i32,
    pub y: i32,
    pub width: u32,
    pub height: u32,
    pub is_full_screen: bool,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowUsage {
    pub memory: u64,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityWindow {
    pub id: u32,
    pub info: WindowInfo,
    pub os: String,
    pub position: WindowPosition,
    pub title: String,
    pub usage: WindowUsage,
    pub url: Option<String>,
}
