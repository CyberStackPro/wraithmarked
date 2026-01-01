use crate::models::{
    ActivityDetails, ActivityEvent, ActivityType, EventType, KeystrokeEvent, KeystrokeStats,
    MinuteStat, MouseButton, ScrollDirection,
};
use chrono::Utc;
use log::{error, info};
use rdev::{listen, Button as RdevButton, Event, EventType as RdevEventType};
use serde_json;
use std::fs::File;
use std::io::Write;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// use super::tracker_trait::Tracker;

const MAX_RECENT_KEYS: usize = 50;
const MAX_RECENT_KEY_EVENTS: usize = 100;
const MAX_MINUTE_HISTORY: usize = 60;
const DATA_FILE_PATH: &str = "activity_data.json";
const DATA_SAVE_INTERVAL_MINUTES: u64 = 5;

// #[derive(Clone)]
pub struct KeystrokeTracker {
    is_tracking: bool,
    recent_keys: Vec<String>,
    pub activity_events: Vec<ActivityEvent>,

    pub stats: KeystrokeStats,
    pub start_time: Option<chrono::DateTime<Utc>>,

    stop_signal: Arc<AtomicBool>,
    monitor_handle: Option<thread::JoinHandle<()>>,
    listener_handle: Option<thread::JoinHandle<()>>,
}

impl KeystrokeTracker {
    pub fn new() -> Self {
        Self {
            is_tracking: false,
            recent_keys: Vec::new(),
            activity_events: Vec::new(),
            stats: KeystrokeStats::default(),
            start_time: None,
            stop_signal: Arc::new(AtomicBool::new(false)),
            listener_handle: None,
            monitor_handle: None,
        }
    }

    pub fn start_tracking(mut self) -> Arc<Mutex<Self>> {
        if self.is_tracking {
            info!("KeystrokeTracker: Already tracking.");
        }
        self.is_tracking = true;
        self.start_time = Some(Utc::now());

        let stop_signal_for_listener = Arc::clone(&self.stop_signal);
        let stop_signal_for_monitor = Arc::clone(&self.stop_signal);

        let tracker_arc = Arc::new(Mutex::new(self));

        let cloned_tracker_for_listener = Arc::clone(&tracker_arc);
        let cloned_tracker_for_monitor = Arc::clone(&tracker_arc);

        let listener_handle = thread::spawn(move || {
            info!("KeystrokeTracker Listener: Starting...");

            let result = listen(move |event| {
                if stop_signal_for_listener.load(Ordering::SeqCst) {
                    static ONCE: std::sync::Once = std::sync::Once::new();
                    ONCE.call_once(|| {
                        info!("Listener: Stop signal received, exiting event handler.");
                    });
                    return;
                }

                if let Ok(mut tracker_guard) = cloned_tracker_for_listener.lock() {
                    tracker_guard.handle_event(event);
                } else {
                    error!("KeystrokeTracker Listener: Failed to acquire lock for event handling (mutex poisoned?).");
                }
            });

            if let Err(err) = result {
                error!(
                    "KeystrokeTracker Listener: Error listening to input events: {:?}",
                    err
                );
            }
            info!("KeystrokeTracker Listener: Thread finished.");
        });

        let monitor_handle = thread::spawn(move || {
            info!("KeystrokeTracker Monitor: Starting periodic monitor thread.");
            let mut last_minute_aggregation_time = Utc::now();
            let mut last_total_keystrokes = 0;
            let mut last_save_time = Utc::now();

            loop {
                if stop_signal_for_monitor.load(Ordering::SeqCst) {
                    info!("KeystrokeTracker Monitor: Stop signal received, exiting.");
                    break;
                }

                thread::sleep(Duration::from_secs(10));

                let now = Utc::now();
                if (now - last_minute_aggregation_time) >= chrono::Duration::seconds(60) {
                    let mut tracker_guard = cloned_tracker_for_monitor.lock().unwrap();

                    let current_total_keystrokes = tracker_guard.stats.total_count; // Use stats.total_count
                    let keystrokes_this_minute = current_total_keystrokes - last_total_keystrokes;

                    tracker_guard.stats.minute_history.push(MinuteStat {
                        timestamp: last_minute_aggregation_time,
                        count: keystrokes_this_minute,
                    });
                    if tracker_guard.stats.minute_history.len() > MAX_MINUTE_HISTORY {
                        tracker_guard.stats.minute_history.remove(0);
                    }

                    if !tracker_guard.stats.recent_events.is_empty() {
                        let oldest_event_time = tracker_guard.stats.recent_events[0].timestamp;
                        let newest_event_time =
                            tracker_guard.stats.recent_events.last().unwrap().timestamp;
                        let duration_secs =
                            (newest_event_time - oldest_event_time).num_seconds() as f64;

                        tracker_guard.stats.recent_rate = if duration_secs > 0.0 {
                            tracker_guard.stats.recent_events.len() as f64 / duration_secs * 60.0
                        } else {
                            0.0
                        };
                    } else {
                        tracker_guard.stats.recent_rate = 0.0;
                    }

                    last_total_keystrokes = current_total_keystrokes;
                    last_minute_aggregation_time = now;
                }
                if (now - last_save_time)
                    >= chrono::Duration::minutes(DATA_SAVE_INTERVAL_MINUTES as i64)
                {
                    let tracker_guard = cloned_tracker_for_monitor.lock().unwrap();
                    info!(
                        "Monitor: {} minutes elapsed, attempting to save activity data.",
                        DATA_SAVE_INTERVAL_MINUTES
                    );
                    let timestamp_str = now.format("%Y%m%d_%H%M%S").to_string();
                    let unique_filename = format!("{}_{}.json", DATA_FILE_PATH, timestamp_str);
                    let save_path = std::path::Path::new(&unique_filename);

                    match serde_json::to_string_pretty(&tracker_guard.activity_events) {
                        Ok(json_data) => match File::create(&save_path) {
                            Ok(mut file) => match file.write_all(json_data.as_bytes()) {
                                Ok(_) => {
                                    info!("Activity data saved successfully to {:?}", save_path)
                                }
                                Err(e) => error!(
                                    "Failed to write activity data to file {:?}: {:?}",
                                    save_path, e
                                ),
                            },
                            Err(e) => error!("Failed to create file {:?}: {:?}", save_path, e),
                        },
                        Err(e) => error!("Failed to serialize activity data: {:?}", e),
                    }
                    drop(tracker_guard);
                    let mut tracker_guard = cloned_tracker_for_monitor.lock().unwrap();
                    tracker_guard.activity_events.clear();
                    info!("Activity events buffer cleared.");
                    last_save_time = now;
                }
            }
            info!("KeystrokeTracker Monitor: Thread finished.");
        });
        {
            let mut tracker_guard = tracker_arc.lock().unwrap();
            tracker_guard.listener_handle = Some(listener_handle);
            tracker_guard.monitor_handle = Some(monitor_handle);
        }

        info!("KeystrokeTracker: All tracking threads started.");
        tracker_arc
    }

    pub fn stop_tracking(&mut self) {
        if !self.is_tracking {
            info!("KeystrokeTracker: Not tracking.");
            return;
        }

        info!("KeystrokeTracker: Signaling threads to stop.");
        self.is_tracking = false;
        self.stop_signal.store(true, Ordering::SeqCst);

        // Join monitor thread first
        if let Some(handle) = self.monitor_handle.take() {
            info!("KeystrokeTracker: Waiting for monitor thread to finish...");
            handle.join().expect("Monitor thread panicked during join!");
            info!("KeystrokeTracker: Monitor thread joined.");
        }

        // The listener thread will not join because rdev::listen is a blocking call.
        // We will rely on std::process::exit(0) to kill the thread gracefully
        // after all cleanup has been done.
        info!("KeystrokeTracker: Listener thread cannot be joined gracefully (rdev::listen is blocking).");
        info!("KeystrokeTracker: Ready for final cleanup and exit.");
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

        self.activity_events.push(wheel_activity);
        self.stats.total_scroll_events += 1;

        // println!(
        //     "Scroll direction: {:?}, Total scrolls: {}",
        //     direction, self.stats.total_scroll_events
        // );
    }

    fn handle_key_press(&mut self, key: rdev::Key) {
        let key_str = format!("{:?}", key);
        let now = Utc::now();

        // let mut cloned_keys = self.recent_keys.clone();
        // cloned_keys.push(key_str.clone());
        // let itr_key: Option<Vec<String>> = Some(cloned_keys);

        let activity = ActivityEvent {
            timestamp: now,
            activity_type: ActivityType::Keyboard,
            details: ActivityDetails {
                key: Some(key_str.clone()),
                event_type: Some(EventType::KeyDown),
                ..Default::default()
            },
        };

        self.recent_keys.push(key_str);
        if self.recent_keys.len() > MAX_RECENT_KEYS {
            self.recent_keys.remove(0);
        }

        self.activity_events.push(activity.clone());
        self.stats.total_count += 1;
        self.stats.recent_events.push(KeystrokeEvent {
            timestamp: now,
            key: activity.details.key.clone().unwrap_or_default(),
        });
        if self.stats.recent_events.len() > MAX_RECENT_KEY_EVENTS {
            self.stats.recent_events.remove(0);
        }

        // println!(
        //     "Key pressed: {:?}, Total keystrokes: {}",
        //     activity.details.key, self.stats.total_count
        // );
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
        let now = Utc::now();
        let mouse_button = match button {
            RdevButton::Left => MouseButton::Left,
            RdevButton::Right => MouseButton::Right,
            RdevButton::Middle => MouseButton::Middle,
            RdevButton::Unknown(_) => MouseButton::Unknown,
        };

        let button_activity = ActivityEvent {
            timestamp: now,
            activity_type: ActivityType::Button,
            details: ActivityDetails {
                mouse_button: Some(mouse_button.clone()),
                event_type: Some(EventType::ButtonPress),
                ..Default::default()
            },
        };

        self.activity_events.push(button_activity);
        self.stats.total_mouse_clicks += 1;

        // info!(
        //     "Mouse Button Press: {:?} (Total Clicks: {})",
        //     mouse_button, self.stats.total_mouse_clicks
        // );
    }

    pub fn save_activity_data_to_file(&self) -> Result<(), std::io::Error> {
        info!("Attempting to save final activity data.");
        let timestamp_str = Utc::now().format("%Y%m%d_%H%M%S").to_string();
        let filename = format!("{}_{}.json", DATA_FILE_PATH, timestamp_str);
        let json_data = serde_json::to_string_pretty(&self.activity_events)?;
        let mut file = File::create(filename)?;
        file.write_all(json_data.as_bytes())?;
        info!("Final activity data saved successfully.");
        Ok(())
    }

    pub fn clear_stats(&mut self) {
        self.recent_keys.clear();
        self.activity_events.clear();
        self.stats = KeystrokeStats::default();
        self.start_time = Some(Utc::now());

        info!("Tracker stats cleared.");
    }

    pub fn print_summary(&self) {
        println!("===== KeystrokeTracker Summary =====");
        println!("Total keystrokes: {}", self.stats.total_count);
        println!("Total mouse clicks: {}", self.stats.total_mouse_clicks);
        println!("Total scroll events: {}", self.stats.total_scroll_events);

        let one_minute_ago = Utc::now() - chrono::Duration::seconds(60);
        let recent_keystrokes_count = self
            .activity_events
            .iter()
            .filter(|event| {
                event.timestamp > one_minute_ago && event.activity_type == ActivityType::Keyboard
            })
            .count();

        println!(
            "Keystrokes in the last 60 seconds: {}",
            recent_keystrokes_count
        );

        if let Some(start_time) = self.start_time {
            let duration = Utc::now() - start_time;
            println!("Tracking duration: {} seconds", duration.num_seconds());
            println!("Tracking duration: {} minutes", duration.num_minutes());

            if duration.num_seconds() > 0 {
                let rate = (self.stats.total_count as f64 * 60.0) / duration.num_seconds() as f64;
                println!("Keystrokes per minute: {:.2}", rate);
            }
        }
        println!("===========================");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keystroke_tracker_construction() {
        let tracker = KeystrokeTracker::new();
        assert!(!tracker.is_tracking);
    }
}
