use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ActivityType {
    Keyboard,
    Mouse,
    Wheel,
    Button,
    Window,
    System,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone)]
#[serde(rename_all = "lowercase")]
pub enum EventType {
    KeyDown,
    KeyUp,
    Click,
    Move,
    Wheel,
    Scroll,
    ButtonPress,
    MouseDown,
    MouseMove,
    MouseWheel,
    WindowFocusChange,
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
