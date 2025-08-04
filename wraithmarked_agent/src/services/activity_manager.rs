// use crate::services::active_window::ActiveWindowTracker;
// use crate::services::keystroke_tracker::KeystrokeTracker;
// use log::{error, info};
// use std::sync::{Arc, Mutex};

// pub struct ActivityManager {
//     // The central keystroke tracker where all events are collected
//     keystroke_tracker: Arc<Mutex<KeystrokeTracker>>,
//     // Other trackers that feed events into the keystroke tracker
//     window_tracker: Arc<Mutex<ActiveWindowTracker>>,
// }

// impl ActivityManager {
//     pub fn new() -> Self {
//         // Create a new instance of the central keystroke tracker
//         let keystroke_tracker_instance = KeystrokeTracker::new();
//         let keystroke_tracker_arc = Arc::new(Mutex::new(keystroke_tracker_instance));

//         // Create an instance of the window tracker
//         let window_tracker_instance = ActiveWindowTracker::new();
//         let window_tracker_arc = Arc::new(Mutex::new(window_tracker_instance));

//         Self {
//             keystroke_tracker: keystroke_tracker_arc,
//             window_tracker: window_tracker_arc,
//         }
//     }

//     pub fn start_all(&mut self) {
//         info!("ActivityManager: Starting all trackers...");
//         info!("ActivityManager: Starting all trackers...");

//         // Temporarily take ownership of the Arc<Mutex> so we can move it into the start_tracking method.
//         // We use .take() on the Option to move the Arc out of the KeystrokeTracker.
//         // This is necessary because start_tracking is designed to consume the Arc<Mutex>.
//         let keystroke_tracker = self.keystroke_tracker.lock().unwrap().start_tracking();

//         // Now, we can put the new Arc back into the manager.
//         self.keystroke_tracker = keystroke_tracker;

//         // Do the same for the window tracker.
//         let window_tracker = self
//             .window_tracker
//             .lock()
//             .unwrap()
//             .start_tracking(Arc::clone(&self.keystroke_tracker));
//         self.window_tracker = window_tracker;

//         info!("ActivityManager: All trackers started.");
//     }

//     pub fn stop_all(&mut self) {
//         info!("ActivityManager: Stopping all trackers...");

//         // Stop the window tracker first
//         if let Ok(mut tracker_guard) = self.window_tracker.lock() {
//             tracker_guard.stop_tracking();
//         } else {
//             error!("ActivityManager: Failed to acquire lock for window tracker during stop.");
//         }

//         // Stop the keystroke tracker last, and save the final data
//         if let Ok(mut tracker_guard) = self.keystroke_tracker.lock() {
//             tracker_guard.stop_tracking();

//             // Perform final save and clear after all trackers have stopped
//             match tracker_guard.save_activity_data_to_file() {
//                 Ok(_) => info!("Final activity data successfully saved on shutdown."),
//                 Err(e) => error!("Failed to save final activity data on shutdown: {:?}", e),
//             }
//             tracker_guard.clear_stats();
//         } else {
//             error!("ActivityManager: Failed to acquire lock for keystroke tracker during stop.");
//         }

//         info!("ActivityManager: All trackers stopped.");
//     }

//     pub fn print_summary_all(&self) {
//         info!("ActivityManager: Printing summary for all trackers...");

//         if let Ok(tracker_guard) = self.keystroke_tracker.lock() {
//             tracker_guard.print_summary();
//         } else {
//             error!("ActivityManager: Failed to acquire lock for keystroke tracker for summary.");
//         }

//         if let Ok(tracker_guard) = self.window_tracker.lock() {
//             tracker_guard.print_summary();
//         } else {
//             error!("ActivityManager: Failed to acquire lock for window tracker for summary.");
//         }
//         info!("ActivityManager: All summaries printed.");
//     }

//     pub fn get_keystroke_tracker(&self) -> Arc<Mutex<KeystrokeTracker>> {
//         Arc::clone(&self.keystroke_tracker)
//     }
// }
