use crate::models::{ActivityDetails, ActivityEvent, ActivityType, EventType, WindowInfo};
use chrono::Utc;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;
use x_win::{empty_entity, get_active_window};

// struct LastWindowInfo {
//     app_name: String,
//     exec_name: String,
//     window_title: String,
//     url: Option<String>,
//     timestamp: chrono::DateTime<Utc>, // When this window became active
// }

struct ActiveWindowTracker {
    is_tracking: bool,
    details: WindowInfo,
}

impl ActiveWindowTracker {
    fn new() -> Self {
        Self {
            is_tracking: false,
            details: WindowInfo {
                // app_name: String::new(),
                // exec_name: String::new(),
                // window_title: String::new(),
                // url: String::new(),
                // timestamp: None,
                exec_name: String::new(),
                name: String::new(),
                path: String::new(),
                process_id: 0,
            },
        }
    }
    pub fn start_tracking(self) {
        // self.is_tracking = true;
        println!("Activity Window tracking is started");
    }
}

pub fn active_window() {
    thread::spawn(monitor_active_window);
}

pub fn monitor_active_window() {
    loop {
        match get_active_window() {
            Ok(win) => {
                // let exec_name = win.info.exec_name;
                // let name = win.info.name;

                // let title = win.title;
                // // let url = win;

                // println!("App: {}", name);
                // println!("Exec_name: {}", exec_name);

                // println!("Title: {}", title);
                // if !url.is_empty() {
                //     println!("🌐 URL: {}", url);
                // }

                println!("All windows Info: {:?}", &win);
            }
            Err(e) => println!("⚠️ Error fetching active window: {:?}", e),
        }

        thread::sleep(Duration::from_secs(5));
    }
}
