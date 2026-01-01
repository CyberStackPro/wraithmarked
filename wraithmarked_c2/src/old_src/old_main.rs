use rand::Rng;
use reqwest::{header, Client};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{interval, sleep, timeout, Duration};

mod tcp_stream;

mod blocking_file_downloader;
mod noneblocking_file_downloader;

use noneblocking_file_downloader::blocking_downloader;

use blocking_file_downloader::file_downloader_blocking;
use tcp_stream::try_connect_with_timeout;

use tcp_stream::check_host;

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

    // match try_connect_with_timeout("192.168.1.9:5432", Duration::from_secs(1)).await {
    //     Ok(_) => println!("connected quickly"),
    //     Err(e) => println!("failed: {}", e),
    // }

    // let hosts = vec![
    //     "192.168.1.9:8081".to_string(),
    //     "1.1.1.1:80".to_string(),
    //     "8.8.8.8:53".to_string(),
    // ];
    //
    // // Concurrency and timeout parameters
    // let max_concurrency = 100;
    // let connect_timeout = Duration::from_secs(1);
    //
    // let sem = Arc::new(Semaphore::new(max_concurrency));
    // let mut handles = Vec::with_capacity(hosts.len());
    //
    // for addr in hosts {
    //     let sem_clone = sem.clone();
    //     let addr_clone = addr.clone();
    //     let dur = connect_timeout;
    //
    //     // spawn one task per host (the heavy connect work is gated by semaphore)
    //     let handle = tokio::spawn(async move { check_host(addr_clone, sem_clone, dur).await });
    //
    //     handles.push(handle);
    // }
    //
    // // collect successes
    // let mut reachable = Vec::new();
    // for h in handles {
    //     match h.await {
    //         Ok((addr, true)) => reachable.push(addr),
    //         Ok((_addr, false)) => { /* unreachable, ignore or log */ }
    //         Err(e) => eprintln!("task join error: {}", e),
    //     }
    // }
    //
    // println!("Reachable hosts:");
    // for r in reachable {
    //     println!(" - {}", r);
    // }
    //
    //
    //   let _ = parse_text().await;

    // let res = post_example(
    //     &Client::new(),
    //     "https://jsonplaceholder.typicode.com/posts/1",
    // )
    // .await;
    //
    //
    // let client = build_client();
    // let url = "https://jsonplaceholdr.typicode.com/posts/1";
    // let res =
    //     fetch_json_with_retries(&client, "https://jsonplaceholder.typicode.com/posts/1").await;
    // println!("Result: {:?}", res);
    // match fetch_json_with_retries(&client, url).await {
    //     Ok(json) => println!("Got JSON: {:#?}", json),
    //     Err(err) => eprintln!("Error: {}", err),
    // }

    // match blocking_downloader().await {
    //     Ok(f) => f,
    //     Err(e) => println!("Displaying blocking Error {}", e),
    // }
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

async fn parse_text() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let url = "https://github.com/";
    let res = client.get(url).send().await?;
    let body = res.text().await?;

    println!("body length: {}", body.len());

    Ok(())
}

async fn post_example(client: &Client, url: &str) -> Result<Value, reqwest::Error> {
    let res = client.get(url).send().await?;
    let json: Value = res.json().await?;

    Ok(json)
}

async fn fetch_json_with_retries(
    client: &reqwest::Client,
    url: &str,
) -> Result<serde_json::Value, String> {
    let max_retries = 3;
    let mut retries = 0;

    let mut delay = 1;

    loop {
        let res = client.get(url).send().await;
        match res {
            Ok(json_resp) => {
                if json_resp.status().is_success() {
                    match json_resp.json::<Value>().await {
                        Ok(json) => return Ok(json),
                        Err(e) => return Err(format!("Faild to parse JSON: {}", e)),
                    }
                } else {
                    return Err(format!("HTTP error: {}", json_resp.status()));
                }
            }
            Err(ref e) => {
                retries += 1;
                if retries > max_retries {
                    return Err(format!("Failed after {} retries: {}", max_retries, e));
                }
                println!("Retry {} after error: {}", retries, e);
                // sleep(Duration::from_secs(2)).await;

                let jitter = rand::thread_rng().gen_range(0..=2);
                let wait_time = delay + jitter;
                println!("Waiting {} seconds before retrying...", wait_time);
                sleep(Duration::from_secs(wait_time)).await;
                delay *= 2;
            }
        }
    }
}

fn build_client() -> Client {
    let mut headers = header::HeaderMap::new();
    headers.insert(
        header::USER_AGENT,
        header::HeaderValue::from_static("WraithMarked/0.1"),
    );

    reqwest::Client::builder()
        .default_headers(headers)
        .connect_timeout(Duration::from_secs(5)) // TCP connect timeout
        .pool_idle_timeout(Duration::from_secs(60)) // connection pool tunable
        .build()
        .unwrap()
}
