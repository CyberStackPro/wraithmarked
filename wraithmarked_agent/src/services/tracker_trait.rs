use crate::services::keystroke_tracker::KeystrokeTracker;
use std::sync::{Arc, Mutex}; // Required for ActiveWindowTracker's start method

// Define the Tracker trait
// `Send` and `Sync` bounds are necessary for `dyn Tracker` to be used with `Arc<Mutex>`
// and moved across thread boundaries safely.
pub trait Tracker: Send + Sync {
    // `start` method:
    // Takes `&self` (meaning `&Arc<Mutex<Self>>`).
    // It's responsible for spawning its internal threads and managing its own `JoinHandle`s and `stop_signal`.
    // It also takes a shared reference to the central KeystrokeTracker if it needs to push events.
    fn start(&self, shared_keystroke_tracker: Arc<Mutex<KeystrokeTracker>>);

    // `stop` method:
    // Takes `&self` (meaning `&Arc<Mutex<Self>>`).
    // It's responsible for signaling its internal threads to stop and joining them.
    fn stop(&self);

    // `print_summary` method:
    // Takes `&self` (meaning `&Arc<Mutex<Self>>`).
    fn print_summary(&self);
}
