mod c2_server;
mod models;

#[tokio::main]
async fn main() {
    println!("===========================================");
    println!("    WraithMarked C2 Server Starting...   ");
    println!("===========================================\n");

    c2_server::start_server().await;
}
