use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct WindowInfo {
    pub title: String,
    pub exec_name: String,
    pub name: String,
    pub path: String,
    pub process_id: u32,
    pub icon: Option<String>,
    pub timestamp: chrono::DateTime<Utc>,
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

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct LoggedWindowInfo {
    pub title: Option<String>,
    pub name: Option<String>,      // App name (e.g., "Google Chrome")
    pub exec_name: Option<String>, // Executable name (e.g., "chrome.exe")
    pub path: Option<String>,      // Executable path
    pub process_id: Option<u32>,   // Process ID
    pub url: Option<String>,       // Browser URL
    pub timestamp: DateTime<Utc>,  // When this specific window state was recorded (became active)
}
