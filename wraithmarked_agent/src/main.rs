mod models;
mod services;

use services::active_window::active_window;
use services::keystroke_tracker::KeystrokeTracker;

fn main() {
    // println!("Hello, world!");
    // active_window();

    let tracker = KeystrokeTracker::new();
    let tracker_handle = tracker.start_tracking();

    loop {
        std::thread::sleep(std::time::Duration::from_secs(60));
        if let Ok(tracker) = tracker_handle.lock() {
            // tracker.start_tracking();
            tracker.print_summary();
        }
    }
}
