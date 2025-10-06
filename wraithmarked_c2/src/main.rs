use tokio::time::{interval, sleep, timeout, Duration};

mod tcp_stream;

use tcp_stream::try_connect_with_timeout;

#[tokio::main]
async fn main() {
    // for port in [22, 80, 443] {
    //     tokio::spawn(async move {
    //         println!("Starting server on port: {}", port);

    //         tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    //         println!("Finished scanning port: {}", port);
    //     });
    // }

    // Keep the main function alive to allow async tasks to run
    // loop {
    // tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;
    // }

    // Handles
    /*  let hosts = vec![
        "192.168.1.1".to_string(),
        "10.0.0.5".to_string(),
        "172.16.0.3".to_string(),
    ];

    let mut handles = Vec::new();

    for host in hosts {
        let handle = tokio::spawn(async move {
            println!("Pinging host: {}", host);
            sleep(Duration::from_secs(1)).await;
            println!("Host {} is reachable", host);
        });

        handles.push(handle);
    }

    // Wait for all tasks to finish
    for handle in handles {
        handle.await.unwrap();
    } */

    // periodic_checker().await;
    //try_connect_with_timeout(addr, dur)
    match try_connect_with_timeout("192.168.1.9:5432", Duration::from_secs(1)).await {
        Ok(_) => println!("connected quickly"),
        Err(e) => println!("failed: {}", e),
    }
}
/*

   let mut ticker = interval(Duration::from_secs(5));

    for i in 0..3 {
        ticker.tick().await;
        println!("Beacon check-in {}", i);
    }
*/

//  Beacon (interval) — periodic check-in
// #[tokio::main]
async fn periodic_checker() {
    let mut ticker = interval(Duration::from_secs(5));

    let beacon_handle = tokio::spawn(async move {
        let mut count = 0;
        loop {
            ticker.tick().await;
            count += 1;
            println!("[beacon] check-in #{count}");
        }
    });

    println!("Press Ctrl+C to stop");
    tokio::signal::ctrl_c().await.unwrap();
    println!("Shutting down...");

    beacon_handle.abort();
    let _ = beacon_handle.await;
    println!("Exited");
}
