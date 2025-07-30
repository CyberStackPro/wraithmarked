use crate::models::{
    ActivityDetails, ActivityEvent, ActivityType, EventType, LoggedWindowInfo, WindowInfo,
};
use crate::services::keystroke_tracker::KeystrokeTracker;
use chrono::Utc;
use log::{error, info};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use x_win::{empty_entity, get_active_window, get_browser_url, get_window_icon};

#[cfg(any(target_os = "windows", target_os = "macos"))]
use x_win::get_browser_url;

pub struct ActiveWindowTracker {
    is_tracking: bool,
    // details: WindowInfo,
}

impl ActiveWindowTracker {
    pub fn new() -> Self {
        Self { is_tracking: false }
    }
    pub fn start_tracking(mut self, tracker_arc: Arc<Mutex<KeystrokeTracker>>) -> Arc<Mutex<Self>> {
        if self.is_tracking {
            info!("[Window Tracking] Already Tracking.");
        }

        self.is_tracking = true;
        info!("[Window Tracking] Starting...");

        let active_window_tracker_arc = Arc::new(Mutex::new(self));

        let cloned_tracker_for_monitor = Arc::clone(&tracker_arc);

        thread::spawn(move || {
            info!("[Window Monitor] Thread Started.");

            let mut last_logged_window_info: Option<LoggedWindowInfo> = None;

            loop {
                match get_active_window() {
                    Ok(current_win) => {
                        let current_logged_win_info = LoggedWindowInfo {
                            title: Some(current_win.title),
                            name: Some(current_win.info.name),
                            exec_name: Some(current_win.info.exec_name),
                            path: Some(current_win.info.path),
                            process_id: Some(current_win.id),
                            url: Some(String::from("On linux url is not available")),
                            timestamp: Utc::now(),
                        };

                        let mut window_changed = false;
                        if let Some(last) = &last_logged_window_info {
                            if last.name != current_logged_win_info.name
                                || last.exec_name != current_logged_win_info.exec_name
                                || last.title != current_logged_win_info.title
                                || last.url != current_logged_win_info.url
                            {
                                window_changed = true;
                            }
                        } else {
                            window_changed = true;
                        }

                        if window_changed {
                            if let Some(last) = last_logged_window_info {
                                let duration_seconds =
                                    (current_logged_win_info.timestamp - last.timestamp)
                                        .num_seconds() as u64;
                                info!(
                                    "[Window Monitor] Window changed from '{:?}' ({}s) to '{:?}'",
                                    last.title.clone().unwrap_or_default(),
                                    duration_seconds,
                                    current_logged_win_info.title.clone().unwrap_or_default()
                                );

                                let prev_window_activity = ActivityEvent {
                                    timestamp: last.timestamp,
                                    activity_type: ActivityType::Window,
                                    details: ActivityDetails {
                                        event_type: Some(EventType::WindowFocusChange),
                                        window_info: Some(last),
                                        duration_active_seconds: Some(duration_seconds),
                                        ..Default::default()
                                    },
                                };

                                if let Ok(mut tracker_guard) = cloned_tracker_for_monitor.lock() {
                                    tracker_guard.activity_events.push(prev_window_activity);
                                } else {
                                    error!("[Window Monitor] Failed to acquire tracker lock for previous window event.");
                                }
                            }
                        }

                        last_logged_window_info = Some(current_logged_win_info.clone());

                        let current_window_activity = ActivityEvent {
                            timestamp: current_logged_win_info.timestamp,
                            activity_type: ActivityType::Window,
                            details: ActivityDetails {
                                event_type: Some(EventType::WindowFocusChange),
                                window_info: Some(current_logged_win_info),
                                duration_active_seconds: None,
                                ..Default::default()
                            },
                        };
                        if let Ok(mut tracker_guard) = cloned_tracker_for_monitor.lock() {
                            tracker_guard.activity_events.push(current_window_activity);
                        } else {
                            error!("[Window Monitor] Failed to acquire tracker lock for current window event.");
                        }
                    }

                    Err(e) => error!("[Window Monitor] Error fetching active window: {:?}", e),
                }
                thread::sleep(Duration::from_secs(1));
            }
        });
        active_window_tracker_arc

        // println!("Activity Window tracking is started");
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

                let active_windows_icon = get_window_icon(&win);

                println!("Active Windows Icon: {:?}", active_windows_icon);
                println!("All windows Info: {:?}", &win);
            }
            Err(e) => println!("⚠️ Error fetching active window: {:?}", e),
        }

        thread::sleep(Duration::from_secs(5));
    }
}
