// pub struct TrackerManager {
//     trackers: Vec<Box<dyn Tracker>>,
// }

// impl TrackerManager {
//     pub fn new() -> Self {
//         Self { trackers: vec![] }
//     }

//     pub fn add_tracker(&mut self, tracker: Box<dyn Tracker>) {
//         self.trackers.push(tracker);
//     }

//     pub fn start_all(&self) {
//         for t in &self.trackers {
//             t.start_tracking();
//         }
//     }

//     pub fn stop_all(&self) {
//         for t in &self.trackers {
//             t.stop_tracking();
//         }
//     }

//     pub fn summary_all(&self) {
//         for t in &self.trackers {
//             t.print_summary();
//         }
//     }
// }
// impl TrackerState {
//     pub fn toggle(&mut self) {
//         self.is_tracking = !self.is_tracking;
//     }

//     pub fn start(&mut self) {
//         self.is_tracking = true;
//     }

//     pub fn stop(&mut self) {
//         self.is_tracking = false;
//     }
// }
