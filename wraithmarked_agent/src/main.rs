mod models;
mod services;

use services::active_window::monitor_active_window;
use services::keystroke_tracker::KeystrokeTracker;

fn main() {
    // println!("Hello, world!");
    // active_window();

    // let tracker = KeystrokeTracker::new();
    // let tracker_handle = tracker.start_tracking();

    loop {
        // std::thread::sleep(std::time::Duration::from_secs(60));
        monitor_active_window();
        // if let Ok(tracker) = tracker_handle.lock() {
        //     // tracker.start_tracking();
        //     tracker.print_summary();
        //     // tracker.save_activity_data_to_file().unwrap();
        // }
    }
}
