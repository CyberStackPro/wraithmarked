use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

// in rust this header that are start with # called Rust attribute macro

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActivityType {
    Keyboard,
    Mouse,
    Wheel,
    Button,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    KeyUp,
    KeyDown,
    MouseMove,
    MouseWheel,
    ButtonPress,
    // Wheel,
    // Click,
    // Move,
    // Scroll,
    // MouseDown,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ScrollDirection {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Unknown,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityDetails {
    pub key: Option<String>,
    pub mouse_button: Option<MouseButton>,
    pub mouse_x: Option<i32>,
    pub mouse_y: Option<i32>,
    pub scroll_direction: Option<ScrollDirection>,
    pub event_type: Option<EventType>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ActivityEvent {
    pub timestamp: DateTime<Utc>,
    pub activity_type: ActivityType,
    pub details: ActivityDetails,
}

// New Added
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteStat {
    pub timestamp: DateTime<Utc>,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntervalKeyStrokeStat {
    interval_keystroke_count: u64,
    interval_mouse_click_count: u64,
    interval_scroll_count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KeystrokeEvent {
    pub timestamp: DateTime<Utc>,
    pub key: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeystrokeStats {
    pub total_count: u64,
    pub recent_rate: f64,
    pub minute_history: Vec<MinuteStat>,
    pub recent_events: Vec<KeystrokeEvent>,
    pub total_mouse_clicks: u64,
    pub total_scroll_events: u64,
}
