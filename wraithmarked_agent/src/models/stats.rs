use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MinuteStat {
    pub timestamp: DateTime<Utc>,
    pub count: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntervalKeyStrokeStat {
    pub interval_keystroke_count: u64,
    pub interval_mouse_click_count: u64,
    pub interval_scroll_count: u64,
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
