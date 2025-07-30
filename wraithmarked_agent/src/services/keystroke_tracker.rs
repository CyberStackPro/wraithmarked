use crate::models::{
    ActivityDetails, ActivityEvent, ActivityType, EventType, KeystrokeStats, MouseButton,
    ScrollDirection,
};
use chrono::Utc;
use log::info;
use rdev::{listen, Button as RdevButton, Event, EventType as RdevEventType};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;

// Arc<T>	Shared ownership (multiple threads can own the same data)
// ---- Arc is required so multiple threads can access the tracker (ownership sharing)

// Mutex<T>	Allows one-at-a-time mutable access to the data
// ------ Mutex is required so they can mutably change it safely, one at a time

// pub trait Tracker {
//     fn start_tracking(self: Arc<Self>) -> Arc<Mutex<Self>>;
//     fn stop_tracking(&self);
//     fn print_summary(&self);
// }

// impl Tracker for KeystrokeTracker {
//     fn start_tracking(self: Arc<Self>) -> Arc<Mutex<Self>> {
//         let tracker = Arc::new(Mutex::new((*self).clone()));

//         {
//             let mut t = tracker.lock().unwrap();
//             if t.is_tracking {
//                 println!("Already Tracking.");
//                 return tracker;
//             }
//             t.is_tracking = true;
//             t.start_time = Some(Utc::now());
//         }

//         let thread_tracker = Arc::clone(&tracker);

//         thread::spawn(move || {
//             let result = listen(move |event| {
//                 if let Ok(mut t) = thread_tracker.lock() {
//                     t.handle_event(event);
//                 } else {
//                     eprintln!("Failed to acquire lock while handling event");
//                 }
//             });

//             if let Err(err) = result {
//                 eprintln!("Error listening to input events: {:?}", err);
//             }
//         });

//         tracker
//     }
//     fn stop_tracking(&self) {
//         println!("Activity is Stopped");
//     }
//     fn print_summary(&self) {
//         println!("===== Activity Summary =====");
//         println!("Total keystrokes: {}", self.total_keystrokes);
//         println!("Total mouse clicks: {}", self.total_mouse_clicks);
//         println!("Total scroll events: {}", self.total_scroll_events);

//         if let Some(start_time) = self.start_time {
//             let duration = Utc::now() - start_time;
//             println!("Tracking duration: {} seconds", duration.num_seconds());

//             if duration.num_seconds() > 0 {
//                 let rate = (self.total_keystrokes as f64 * 60.0) / duration.num_seconds() as f64;
//                 println!("Keystrokes per minute: {:.2}", rate);
//             }
//         }
//         println!("===========================");
//     }
// }

const MAX_RECENT_KEYS: usize = 50;
const MAX_RECENT_KEY_EVENTS: usize = 100;
const MAX_MINUTE_HISTORY: usize = 60;
const DATA_FILE_PATH: &str = "activity_data.json";

// #[derive(Clone)]
pub struct KeystrokeTracker {
    is_tracking: bool,
    recent_keys: Vec<String>,
    activity_events: Vec<ActivityEvent>,

    total_mouse_clicks: u64,
    total_scroll_events: u64,
    total_keystrokes: u64,

    start_time: Option<chrono::DateTime<Utc>>,

    stop_signal: Arc<AtomicBool>,
    listener_handle: Option<thread::JoinHandle<()>>,
    monitor_handle: Option<thread::JoinHandle<()>>,
}

impl KeystrokeTracker {
    pub fn new() -> Self {
        Self {
            is_tracking: false,
            recent_keys: Vec::new(),
            activity_events: Vec::new(),
            total_keystrokes: 0,
            total_mouse_clicks: 0,
            total_scroll_events: 0,
            start_time: None,
            stop_signal: Arc::new(AtomicBool::new(false)),
            listener_handle: None,
            monitor_handle: None,
        }
    }

    pub fn start_tracking(self) -> Arc<Mutex<Self>> {
        let tracker = Arc::new(Mutex::new(self));
        tracker.lock().unwrap().is_tracking = true;
        tracker.lock().unwrap().start_time = Some(Utc::now());

        let cloned_tracker = Arc::clone(&tracker);

        thread::spawn(move || {
            let result = listen(move |event| {
                if let Ok(mut tracker) = cloned_tracker.lock() {
                    tracker.handle_event(event);
                }

                // match cloned_tracker.lock() {
                //     Ok(mut tracker) => {
                //         tracker.handle_event(event);
                //     }
                //     Err(err) => {
                //         // Do nothing or log an error
                //         eprintln!("Error listening to input events: {:?}", err);
                //     }
                // }
            });

            if let Err(err) = result {
                eprintln!("Error listening to input events: {:?}", err);
            }
        });
        tracker

        // loop {
        //     std::thread::sleep(std::time::Duration::from_secs(60));
        // }
    }

    fn handle_event(&mut self, event: Event) {
        match event.event_type {
            RdevEventType::KeyPress(key) => self.handle_key_press(key),
            RdevEventType::Wheel { delta_x, delta_y } => self.handle_wheel_event(delta_x, delta_y),
            RdevEventType::MouseMove { x, y } => self.handle_mouse_move(x, y),
            RdevEventType::ButtonPress(button) => self.handle_button_press(button),
            _ => {}
        }
    }

    fn handle_wheel_event(&mut self, delta_x: i64, delta_y: i64) {
        let direction = match (delta_x, delta_y) {
            (_, dy) if dy > 0 => Some(ScrollDirection::Up),
            (_, dy) if dy < 0 => Some(ScrollDirection::Down),
            (dx, _) if dx > 0 => Some(ScrollDirection::Left),
            (dx, _) if dx < 0 => Some(ScrollDirection::Right),
            _ => None,
        };

        let wheel_activity = ActivityEvent {
            timestamp: Utc::now(),
            activity_type: ActivityType::Wheel,
            details: ActivityDetails {
                scroll_direction: direction.clone(),
                event_type: Some(EventType::MouseWheel),
                ..Default::default()
            },
        };

        self.total_scroll_events += 1;

        self.activity_events.push(wheel_activity);

        println!(
            "Scroll direction: {:?}, Total scrolls: {}",
            direction, self.total_scroll_events
        );
    }

    fn handle_key_press(&mut self, key: rdev::Key) {
        let key_str = format!("{:?}", key);

        let mut cloned_keys = self.recent_keys.clone();
        cloned_keys.push(key_str.clone());
        let itr_key: Option<Vec<String>> = Some(cloned_keys);

        let activity = ActivityEvent {
            timestamp: Utc::now(),
            activity_type: ActivityType::Keyboard,
            details: ActivityDetails {
                key: Some(key_str.clone()),
                recent_keys: itr_key,
                event_type: Some(EventType::KeyDown),
                ..Default::default()
            },
        };

        // updating
        self.recent_keys.push(key_str);
        if self.recent_keys.len() > MAX_RECENT_KEYS {
            self.recent_keys.remove(0);
        }

        self.total_keystrokes += 1;

        self.activity_events.push(activity.clone());

        println!(
            "Key pressed: {:?}, Total keystrokes: {}",
            activity.details.key, self.total_keystrokes
        );
    }

    fn handle_mouse_move(&mut self, x: f64, y: f64) {
        let mouse_activity = ActivityEvent {
            timestamp: Utc::now(),
            activity_type: ActivityType::Mouse,
            details: ActivityDetails {
                mouse_x: Some(x as i32),
                mouse_y: Some(y as i32),
                event_type: Some(EventType::MouseMove),
                ..Default::default()
            },
        };

        self.activity_events.push(mouse_activity);
    }

    fn handle_button_press(&mut self, button: RdevButton) {
        let mouse_button = match button {
            RdevButton::Left => MouseButton::Left,
            RdevButton::Right => MouseButton::Right,
            RdevButton::Middle => MouseButton::Middle,
            RdevButton::Unknown(_) => MouseButton::Unknown,
        };

        let button_activity = ActivityEvent {
            timestamp: Utc::now(),
            activity_type: ActivityType::Button,
            details: ActivityDetails {
                mouse_button: Some(mouse_button.clone()),
                scroll_direction: None,
                event_type: Some(EventType::ButtonPress),
                ..Default::default()
            },
        };

        // Update statistics
        self.total_mouse_clicks += 1;

        // Save activity event
        self.activity_events.push(button_activity);

        println!(
            "Mouse button pressed: {:?}, Total clicks: {}",
            mouse_button, self.total_mouse_clicks
        );
    }
    pub fn stop_tracking(&mut self) {
        self.stop_signal.store(true, Ordering::Relaxed);
        if let Some(handle) = self.listener_handle.take() {
            let _ = handle.join();
        }
        if let Some(handle) = self.monitor_handle.take() {
            let _ = handle.join();
        }
        self.is_tracking = false;
    }

    pub fn save_activity_data_to_file(&self) -> Result<(), std::io::Error> {
        info!("Attempting to save activity data to {}", DATA_FILE_PATH);
        let json_data = serde_json::to_string_pretty(&self.activity_events)?; // Pretty print for readability
        let mut file = File::create(DATA_FILE_PATH)?;
        file.write_all(json_data.as_bytes())?;
        info!("Activity data saved successfully to {}", DATA_FILE_PATH);
        Ok(())
    }
    pub fn clear_stats(&mut self) {
        self.recent_keys.clear();
        self.activity_events.clear();
        self.total_keystrokes = 0;
        self.total_mouse_clicks = 0;
        self.total_scroll_events = 0;
        self.start_time = Some(Utc::now()); // Reset start time
        info!("Tracker stats cleared.");
        println!("Tracker stats cleared.");
    }
    pub fn print_summary(&self) {
        println!("===== Activity Summary =====");
        println!("Total keystrokes: {}", self.total_keystrokes);
        println!("Total mouse clicks: {}", self.total_mouse_clicks);
        println!("Total scroll events: {}", self.total_scroll_events);

        // let one_minute_ago = Utc::now() - chrono::Duration::seconds(60);
        // let recent_keystrokes: Vec<String> = self
        //     .activity_events
        //     .iter()
        //     .filter(|event| {
        //         event.timestamp > one_minute_ago && event.activity_type == ActivityType::Keyboard
        //     })
        //     .filter_map(|event| event.details.key.clone())
        //     .collect();

        // println!(
        //     "Keystrokes in the last 60 seconds: {}",
        //     recent_keystrokes.len()
        // );
        // if !recent_keystrokes.is_empty() {
        //     let formatted_keys: Vec<String> =
        //         recent_keystrokes.iter().map(|k| k.to_string()).collect();

        //     println!("Keys: [{}]", formatted_keys.join(", "));
        // }

        if let Some(start_time) = self.start_time {
            let duration = Utc::now() - start_time;
            println!("Tracking duration: {} seconds", duration.num_seconds());
            println!("Tracking duration: {} minutes", duration.num_minutes());

            if duration.num_seconds() > 0 {
                let rate = (self.total_keystrokes as f64 * 60.0) / duration.num_seconds() as f64;
                println!("Keystrokes per minute: {:.2}", rate);
            }
        }
        println!("===========================");
    }
}

// pub struct TrackerManager {
//     trackers: Vec<Box<dyn Tracker>>,
// }

// impl TrackerManager {
//     pub fn new() -> Self {
//         Self { trackers: vec![] }
//     }

//     pub fn add_tracker(&mut self, tracker: Box<dyn Tracker>) {
//         self.trackers.push(tracker);
//     }

//     pub fn start_all(&self) {
//         for t in &self.trackers {
//             t.start_tracking();
//         }
//     }

//     pub fn stop_all(&self) {
//         for t in &self.trackers {
//             t.stop_tracking();
//         }
//     }

//     pub fn summary_all(&self) {
//         for t in &self.trackers {
//             t.print_summary();
//         }
//     }
// }
// impl TrackerState {
//     pub fn toggle(&mut self) {
//         self.is_tracking = !self.is_tracking;
//     }

//     pub fn start(&mut self) {
//         self.is_tracking = true;
//     }

//     pub fn stop(&mut self) {
//         self.is_tracking = false;
//     }
// }

// Example usage to construct KeystrokeTracker
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystroke_tracker_construction() {
        let tracker = KeystrokeTracker::new();
        assert!(!tracker.is_tracking);
    }
}
