use std::{thread, time::Duration};
use x_win::get_active_window;

struct ActiveWindowTracker {
    is_tracking: bool,
}

pub fn active_window() {
    thread::spawn(monitor_active_window);
}

fn monitor_active_window() {
    loop {
        match get_active_window() {
            Ok(win) => {
                let exec_name = win.info.exec_name;
                let name = win.info.name;

                let title = win.title;
                // let url = win;

                println!("App: {}", name);
                println!("Exec_name: {}", exec_name);

                println!("Title: {}", title);
                // if !url.is_empty() {
                //     println!("🌐 URL: {}", url);
                // }

                // println!("All windows Info: ", &win);
            }
            Err(e) => println!("⚠️ Error fetching active window: {:?}", e),
        }

        thread::sleep(Duration::from_secs(5));
    }
}
