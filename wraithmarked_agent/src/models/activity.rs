use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

use crate::models::{ActivityWindow, LoggedWindowInfo, WindowInfo, WindowPosition, WindowUsage};

use super::enums::{ActivityType, EventType, MouseButton, ScrollDirection};

#[derive(Debug, Default, Serialize, Deserialize, Clone)]
pub struct ActivityDetails {
    pub key: Option<String>,
    pub mouse_button: Option<MouseButton>,
    pub recent_keys: Option<Vec<String>>,
    pub mouse_x: Option<i32>,
    pub mouse_y: Option<i32>,
    pub scroll_direction: Option<ScrollDirection>,
    pub event_type: Option<EventType>,

    // pub app_name: Option<String>,
    // pub exec_name: Option<String>,
    // pub window_title: Option<String>,
    // pub url: Option<String>,
    pub window_info: Option<LoggedWindowInfo>,
    pub duration_active_seconds: Option<u64>,
    // pub position: Option<WindowPosition>,
    // pub usage: Option<WindowUsage>,
    // pub full_window: Option<ActivityWindow>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityEvent {
    pub timestamp: DateTime<Utc>,
    pub activity_type: ActivityType,
    pub details: ActivityDetails,
}
