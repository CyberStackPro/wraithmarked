// use serde::Serialize;
// use std::fs;
// use std::path::PathBuf;
// use warp::Filter;

// #[derive(Serialize)]
// struct FileEntry {
//     name: String,
//     is_dir: bool,
//     size: Option<u64>,
// }

// async fn list_directory(path: PathBuf) -> Result<impl warp::Reply, warp::Rejection> {
//     let entries = match fs::read_dir(&path) {
//         Ok(read_dir) => {
//             let mut list = vec![];
//             for entry in read_dir.flatten() {
//                 let metadata = match entry.metadata() {
//                     Ok(m) => m,
//                     Err(_) => continue,
//                 };

//                 list.push(FileEntry {
//                     name: entry.file_name().to_string_lossy().into(),
//                     is_dir: metadata.is_dir(),
//                     size: if metadata.is_file() {
//                         Some(metadata.len())
//                     } else {
//                         None
//                     },
//                 });
//             }
//             list
//         }
//         Err(_) => vec![],
//     };

//     Ok(warp::reply::json(&entries))
// }

// #[tokio::main]
// async fn main() {
//     let files_route = warp::path("list")
//         .and(warp::query::<std::collections::HashMap<String, String>>())
//         .and_then(|params: std::collections::HashMap<String, String>| {
//             let path_str = params.get("path").map(|s| s.as_str()).unwrap_or(".");
//             let path = std::path::PathBuf::from(path_str);
//             list_directory(path)
//         });

//     println!("Server running at http://localhost:3030/list/<relative_path>");
//     warp::serve(files_route).run(([127, 0, 0, 1], 3030)).await;
// }

// use windows_service::{
//     define_windows_service,
//     service::{
//         ServiceControl, ServiceControlAccept, ServiceExitCode, ServiceState, ServiceStatus,
//         ServiceStatusHandle, ServiceType,
//     },
//     service_control_handler::{self, ServiceControlHandlerResult},
//     service_dispatcher,
// };

// use std::{thread, time::Duration, sync::{mpsc, Arc, Mutex}};
// use log::{info, error};
// use simplelog::{WriteLogger, Config, LevelFilter};
// use std::fs::File;
// use chrono::Local;

// const SERVICE_NAME: &str = "MyRustService";

// define_windows_service!(ffi_service_main, my_service_main);

// fn main() -> windows_service::Result<()> {
//     // Start the service dispatcher.
//     service_dispatcher::start(SERVICE_NAME, ffi_service_main)?;
//     Ok(())
// }

// fn my_service_main(_arguments: Vec<std::ffi::OsString>) {
//     // Initialize logger
//     WriteLogger::init(
//         LevelFilter::Info,
//         Config::default(),
//         File::create("C:\\temp\\my_rust_service.log").unwrap(),
//     ).unwrap();

//     info!("Service is starting.");

//     if let Err(e) = run_service() {
//         error!("Service error: {:?}", e);
//     }

//     info!("Service has stopped.");
// }

// fn run_service() -> windows_service::Result<()> {
//     let (shutdown_tx, shutdown_rx) = mpsc::channel();

//     // Register system service control handler
//     let status_handle = service_control_handler::register(SERVICE_NAME, move |control_event| {
//         match control_event {
//             ServiceControl::Stop | ServiceControl::Interrogate => {
//                 shutdown_tx.send(()).unwrap();
//                 ServiceControlHandlerResult::NoError
//             }
//             _ => ServiceControlHandlerResult::NotImplemented,
//         }
//     })?;

//     // Tell the system service is running
//     status_handle.set_service_status(ServiceStatus {
//         service_type: ServiceType::OWN_PROCESS,
//         current_state: ServiceState::Running,
//         controls_accepted: ServiceControlAccept::STOP,
//         exit_code: ServiceExitCode::Win32(0),
//         checkpoint: 0,
//         wait_hint: Duration::default(),
//         process_id: None,
//     })?;

//     info!("Service is running. Waiting for stop signal...");

//     // Main work loop — for example, log current time every 5 seconds
//     loop {
//         // Non-blocking check if stop signal received
//         if let Ok(_) = shutdown_rx.try_recv() {
//             info!("Stop signal received.");
//             break;
//         }

//         let now = Local::now();
//         info!("Heartbeat at {}", now.format("%Y-%m-%d %H:%M:%S"));

//         thread::sleep(Duration::from_secs(5));
//     }

//     // Tell the system service is stopped
//     status_handle.set_service_status(ServiceStatus {
//         service_type: ServiceType::OWN_PROCESS,
//         current_state: ServiceState::Stopped,
//         controls_accepted: ServiceControlAccept::empty(),
//         exit_code: ServiceExitCode::Win32(0),
//         checkpoint: 0,
//         wait_hint: Duration::default(),
//         process_id: None,
//     })?;

//     Ok(())
// }
