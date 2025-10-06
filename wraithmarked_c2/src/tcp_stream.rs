use tokio::net::TcpStream;
use tokio::time::{timeout, Duration};

// Newly added
use std::sync::Arc;
use tokio::sync::Semaphore;

pub async fn try_connect_with_timeout(addr: &str, dur: Duration) -> Result<(), String> {
    match timeout(dur, TcpStream::connect(addr)).await {
        Ok(Ok(stream)) => {
            // success
            println!("Connection Stream: {:?}", stream);
            drop(stream);
            Ok(())
        }
        Ok(Err(e)) => Err(format!("connect error: {}", e)),
        Err(_) => Err("timed out".into()),
    }
}

pub async fn check_host(addr: String, sem: Arc<Semaphore>) -> (String, bool) {
    // acquire permit (async)
    let permit = sem.acquire().await.unwrap();
    let result = match timeout(Duration::from_secs(1), TcpStream::connect(&addr)).await {
        Ok(Ok(_stream)) => true,
        _ => false,
    };
    drop(permit); // release
    (addr, result)
}
