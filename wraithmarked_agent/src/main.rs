mod models;
mod services;

use services::active_window::monitor_active_window;
use services::keystroke_tracker::KeystrokeTracker;

use ctrlc;
use log::{error, info};
use std::time::Duration;

use std::sync::{Arc, Mutex};
use std::thread;

use crate::services::active_window::ActiveWindowTracker;

fn main() {
    env_logger::init();

    info!("--- WraithMarked Activity Monitor Application Starting ---");

    let my_keystroke_tracker = KeystrokeTracker::new();

    let shared_keystroke_tracker: Arc<Mutex<KeystrokeTracker>> =
        my_keystroke_tracker.start_tracking();

    let my_active_window_tracker = ActiveWindowTracker::new();

    let _shared_active_window_tracker: Arc<Mutex<ActiveWindowTracker>> =
        my_active_window_tracker.start_tracking(Arc::clone(&shared_keystroke_tracker));

    let shutdown_keystroke_tracker_clone = Arc::clone(&shared_keystroke_tracker);

    // ctrlc::set_handler(move || {
    //     info!("Ctrl+C received. Initiating graceful shutdown...");
    //     if let Ok(mut tracker_guard) = shutdown_keystroke_tracker_clone.lock() {
    //         tracker_guard.stop_tracking(); // This should join internal threads
    //         match tracker_guard.save_activity_data_to_file() {
    //             Ok(_) => info!("Data successfully saved on Ctrl+C."),
    //             Err(e) => error!("Failed to save data on Ctrl+C: {:?}", e),
    //         }
    //         tracker_guard.clear_stats();
    //     } else {
    //         error!("Failed to acquire keystroke tracker lock for shutdown during Ctrl+C (mutex poisoned?). Data might not be saved.");
    //     }
    //     std::process::exit(0);
    // }).expect("Error setting Ctrl-C handler");

    // Main application loop
    info!("Main thread active. Monitoring user activity. Press Ctrl+C to stop.");
    loop {
        thread::sleep(Duration::from_secs(10)); // Sleep for a bit

        info!("\n--- Main Thread: Requesting Summary ---");
        if let Ok(tracker_guard) = shared_keystroke_tracker.lock() {
            tracker_guard.print_summary();
            tracker_guard.save_activity_data_to_file().unwrap();
        } else {
            error!("Main Thread: Could not acquire keystroke tracker lock to print summary (mutex poisoned?).");
        }
    }
}
