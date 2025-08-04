use crate::models::{ActivityDetails, ActivityEvent, ActivityType, EventType, LoggedWindowInfo};
use crate::services::keystroke_tracker::KeystrokeTracker;
use chrono::{DateTime, Utc};
use log::{error, info};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::{thread, time::Duration};
use x_win::{get_active_window, WindowInfo as XWinWindowInfo};

// #[cfg(any(target_os = "windows", target_os = "macos"))]
use x_win::get_browser_url;

pub struct ActiveWindowTracker {
    is_tracking: bool,
    stop_signal: Arc<AtomicBool>,
    monitor_handle: Option<thread::JoinHandle<()>>,
}

impl ActiveWindowTracker {
    pub fn new() -> Self {
        Self {
            is_tracking: false,
            stop_signal: Arc::new(AtomicBool::new(false)),
            monitor_handle: None,
        }
    }

    pub fn start_tracking(
        mut self,
        shared_keystroke_tracker: Arc<Mutex<KeystrokeTracker>>,
    ) -> Arc<Mutex<Self>> {
        if self.is_tracking {
            info!("[Window Tracking] Already Tracking.");
            return Arc::new(Mutex::new(self));
        }

        self.is_tracking = true;
        info!("[Window Tracking] Starting...");

        let stop_signal_for_monitor = Arc::clone(&self.stop_signal);
        let cloned_tracker_for_monitor = Arc::clone(&shared_keystroke_tracker);
        let this_tracker_arc = Arc::new(Mutex::new(self));
        let cloned_this_tracker_arc = Arc::clone(&this_tracker_arc);

        let monitor_handle = thread::spawn(move || {
            info!("[Window Monitor] Thread Started.");
            let mut last_logged_window_info: Option<LoggedWindowInfo> = None;

            loop {
                if stop_signal_for_monitor.load(Ordering::SeqCst) {
                    info!("[Window Monitor] Stop signal received, exiting.");
                    break;
                }

                match get_active_window() {
                    Ok(current_xwin_info) => {
                        let current_url = Self::get_url_for_platform(&current_xwin_info);
                        let current_logged_win_info = LoggedWindowInfo {
                            title: Some(current_xwin_info.title),
                            name: Some(current_xwin_info.info.name),
                            exec_name: Some(current_xwin_info.info.exec_name),
                            path: Some(current_xwin_info.info.path),
                            process_id: Some(current_xwin_info.id),
                            url: current_url,
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
                    }
                    Err(e) => error!("[Window Monitor] Error fetching active window: {:?}", e),
                }
                thread::sleep(Duration::from_secs(1));
            }
        });

        {
            let mut tracker_guard = this_tracker_arc.lock().unwrap();
            tracker_guard.monitor_handle = Some(monitor_handle);
        }

        this_tracker_arc
    }

    pub fn stop_tracking(&mut self) {
        if !self.is_tracking {
            info!("[Window Tracking] Not tracking.");
            return;
        }

        info!("[Window Tracking] Signaling thread to stop.");
        self.is_tracking = false;
        self.stop_signal.store(true, Ordering::SeqCst);

        if let Some(handle) = self.monitor_handle.take() {
            info!("[Window Tracking] Waiting for monitor thread to finish...");
            handle
                .join()
                .expect("Window monitor thread panicked during join!");
            info!("[Window Tracking] Monitor thread joined.");
        }

        info!("[Window Tracking] Stopped tracking.");
    }

    pub fn print_summary(&self) {
        info!("ActiveWindowTracker: No specific summary to print, events are pushed to KeystrokeTracker.");
    }

    fn get_url_for_platform(window_info: &XWinWindowInfo) -> Option<String> {
        #[cfg(any(target_os = "windows", target_os = "macos"))]
        {
            if let Ok(url) = x_win::get_browser_url(window_info) {
                if !url.is_empty() {
                    return Some(url);
                }
            }
        }
        None
    }
}
