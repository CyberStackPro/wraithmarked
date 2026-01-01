#![allow(unused)]

use axum;

use axum::Router;
use axum::extract::{Path, Query};
use axum::response::{Html, IntoResponse};
use axum::routing::get;

use rand::Rng;
use reqwest::{Client, header};
use serde::Deserialize;
use serde_json::Value;

use std::net::SocketAddr;
use std::sync::Arc;
use tokio::sync::Semaphore;
use tokio::time::{Duration, interval, sleep, timeout};

mod tcp_stream;

mod blocking_file_downloader;
mod noneblocking_file_downloader;

use noneblocking_file_downloader::blocking_downloader;

use blocking_file_downloader::file_downloader_blocking;
use tcp_stream::try_connect_with_timeout;

use tcp_stream::check_host;

#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

#[tokio::main]
async fn main() {
    let route_all = Router::new().merge(routes_hello());
    let addr = SocketAddr::from(([127, 0, 0, 1], 8080));
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();

    println!("->> LISTENING on http://{addr}\n");

    axum::serve(listener, route_all).await.unwrap();
    println!("->> LISTENING on {addr}\n");
}

fn routes_hello() -> Router {
    Router::new()
        .route("/hello", get(handler_hello))
        .route("/hello2/{name}", get(handler_hello2))
}

async fn handler_hello(param: Query<HelloParams>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello -- {param:?}", "HANDLER");
    let name = param.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse {
    println!("->> {:<12} - handler_hello -- {name:?}", "HANDLER");
    // let name = name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
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
// async fn periodic_checker() {
//     let mut ticker = interval(Duration::from_secs(5));

//     let beacon_handle = tokio::spawn(async move {
//         let mut count = 0;
//         loop {
//             ticker.tick().await;
//             count += 1;
//             println!("[beacon] check-in #{count}");
//         }
//     });

//     println!("Press Ctrl+C to stop");
//     tokio::signal::ctrl_c().await.unwrap();
//     println!("Shutting down...");

//     beacon_handle.abort();
//     let _ = beacon_handle.await;
//     println!("Exited");
// }

// async fn parse_text() -> Result<(), reqwest::Error> {
//     let client = Client::new();
//     let url = "https://github.com/";
//     let res = client.get(url).send().await?;
//     let body = res.text().await?;

//     println!("body length: {}", body.len());

//     Ok(())
// }

// async fn post_example(client: &Client, url: &str) -> Result<Value, reqwest::Error> {
//     let res = client.get(url).send().await?;
//     let json: Value = res.json().await?;

//     Ok(json)
// }

// async fn fetch_json_with_retries(
//     client: &reqwest::Client,
//     url: &str,
// ) -> Result<serde_json::Value, String> {
//     let max_retries = 3;
//     let mut retries = 0;

//     let mut delay = 1;

//     loop {
//         let res = client.get(url).send().await;
//         match res {
//             Ok(json_resp) => {
//                 if json_resp.status().is_success() {
//                     match json_resp.json::<Value>().await {
//                         Ok(json) => return Ok(json),
//                         Err(e) => return Err(format!("Faild to parse JSON: {}", e)),
//                     }
//                 } else {
//                     return Err(format!("HTTP error: {}", json_resp.status()));
//                 }
//             }
//             Err(ref e) => {
//                 retries += 1;
//                 if retries > max_retries {
//                     return Err(format!("Failed after {} retries: {}", max_retries, e));
//                 }
//                 println!("Retry {} after error: {}", retries, e);
//                 // sleep(Duration::from_secs(2)).await;

//                 let jitter = rand::thread_rng().gen_range(0..=2);
//                 let wait_time = delay + jitter;
//                 println!("Waiting {} seconds before retrying...", wait_time);
//                 sleep(Duration::from_secs(wait_time)).await;
//                 delay *= 2;
//             }
//         }
//     }
// }

// fn build_client() -> Client {
//     let mut headers = header::HeaderMap::new();
//     headers.insert(
//         header::USER_AGENT,
//         header::HeaderValue::from_static("WraithMarked/0.1"),
//     );

//     reqwest::Client::builder()
//         .default_headers(headers)
//         .connect_timeout(Duration::from_secs(5)) // TCP connect timeout
//         .pool_idle_timeout(Duration::from_secs(60)) // connection pool tunable
//         .build()
//         .unwrap()
// }
