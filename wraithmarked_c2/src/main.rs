// File: wraithmarked_c2/src/main.rs

use comfy_table::modifiers::UTF8_ROUND_CORNERS;
use comfy_table::{Attribute, Cell, Color, ContentArrangement, Table};
use rustyline::DefaultEditor;
use rustyline::error::ReadlineError;
use std::collections::HashMap;
use std::io::Write;
use std::time::{SystemTime, UNIX_EPOCH};

// Enhanced Agent struct with more realistic fields
struct Agent {
    id: String,
    last_seen: String,
    os: String,
    user: String,
    hostname: String,
    ip_address: String,
    privileges: String,
    version: String,
    active_directory: bool,
    sleep: u32, // Sleep time in seconds
}

// File struct for simulated filesystem
struct File {
    name: String,
    size: usize,
    created: String,
    content_type: String,
}

// Process struct for simulated running processes
struct Process {
    pid: u32,
    name: String,
    user: String,
    cpu: f32,
    memory: f32,
}

// Structure to hold current session state
struct Session {
    current_agent: Option<String>,
    current_directory: String,
}

fn main() -> rustyline::Result<()> {
    // Display ASCII art on startup
    display_ascii_art();

    // Initialize a Rustyline editor for the interactive shell
    let mut rl = DefaultEditor::new()?;
    let history_file = "c2_history.txt";
    if rl.load_history(history_file).is_err() {
        println!("No previous history found.");
    }

    // Create session state
    let mut session = Session {
        current_agent: None,
        current_directory: "/home".to_string(),
    };

    // Simulate generating filesystems for agents
    let filesystems = generate_sample_filesystems();

    // Generate sample processes for agents
    let processes = generate_sample_processes();

    // Main loop for the CLI
    loop {
        // Create a dynamic prompt that shows current agent if one is selected
        let prompt = match &session.current_agent {
            Some(agent_id) => format!(
                "wraithmarked({})[{}]> ",
                agent_id, session.current_directory
            ),
            None => "wraithmarked> ".to_string(),
        };

        let readline = rl.readline(&prompt);

        match readline {
            Ok(line) => {
                rl.add_history_entry(line.as_str())?;

                // Split the command into parts for more complex parsing
                let parts: Vec<&str> = line.trim().split_whitespace().collect();
                if parts.is_empty() {
                    continue;
                }

                let command = parts[0];
                let args = &parts[1..];

                match command {
                    "help" => {
                        display_help();
                    }
                    "agents" => {
                        println!("Listing active agents...");
                        let agents = generate_sample_agents();
                        let table = create_agents_table(agents);
                        println!("{table}");
                    }
                    "use" => {
                        if args.is_empty() {
                            println!("Usage: use <agent_id>");
                        } else {
                            let agent_id = args[0];
                            let agents = generate_sample_agents();
                            if agents.iter().any(|a| a.id == agent_id) {
                                session.current_agent = Some(agent_id.to_string());
                                println!("Now controlling agent: {}", agent_id);
                            } else {
                                println!("Agent {} not found", agent_id);
                            }
                        }
                    }
                    "back" => {
                        session.current_agent = None;
                        println!("Returned to main console");
                    }
                    "ls" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        let agent_id = session.current_agent.as_ref().unwrap();
                        let path = if args.is_empty() {
                            &session.current_directory
                        } else {
                            args[0]
                        };

                        list_files(agent_id, path, &filesystems);
                    }
                    "cd" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        if args.is_empty() {
                            println!("Usage: cd <directory>");
                        } else {
                            let new_dir = args[0];
                            // Simple directory change simulation
                            if new_dir == ".." {
                                let parts: Vec<&str> =
                                    session.current_directory.split('/').collect();
                                if parts.len() > 1 {
                                    session.current_directory = parts[0..parts.len() - 1].join("/");
                                    if session.current_directory.is_empty() {
                                        session.current_directory = "/".to_string();
                                    }
                                }
                            } else if new_dir.starts_with('/') {
                                session.current_directory = new_dir.to_string();
                            } else {
                                if session.current_directory.ends_with('/') {
                                    session.current_directory =
                                        format!("{}{}", session.current_directory, new_dir);
                                } else {
                                    session.current_directory =
                                        format!("{}/{}", session.current_directory, new_dir);
                                }
                            }
                            println!("Changed directory to: {}", session.current_directory);
                        }
                    }
                    "shell" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        if args.is_empty() {
                            println!("Usage: shell <command>");
                        } else {
                            let command = args.join(" ");
                            println!("Executing on agent: {}", command);
                            simulate_shell_command(&command);
                        }
                    }
                    "download" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        if args.is_empty() {
                            println!("Usage: download <remote_file> [local_path]");
                        } else {
                            let remote_file = args[0];
                            let local_path = if args.len() > 1 { args[1] } else { "." };
                            println!("Downloading {} to {}...", remote_file, local_path);

                            // Simulate download progress
                            std::io::stdout().write_all(b"Progress: [").unwrap();
                            for _ in 0..20 {
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                std::io::stdout().write_all(b"=").unwrap();
                                std::io::stdout().flush().unwrap();
                            }
                            println!("] 100%");
                            println!("Download complete: {}", remote_file);
                        }
                    }
                    "upload" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        if args.is_empty() || args.len() < 2 {
                            println!("Usage: upload <local_file> <remote_path>");
                        } else {
                            let local_file = args[0];
                            let remote_path = args[1];
                            println!("Uploading {} to {}...", local_file, remote_path);

                            // Simulate upload progress
                            std::io::stdout().write_all(b"Progress: [").unwrap();
                            for _ in 0..20 {
                                std::thread::sleep(std::time::Duration::from_millis(50));
                                std::io::stdout().write_all(b"=").unwrap();
                                std::io::stdout().flush().unwrap();
                            }
                            println!("] 100%");
                            println!("Upload complete: {}", local_file);
                        }
                    }
                    "screenshot" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        println!("Capturing screenshot from agent...");
                        std::thread::sleep(std::time::Duration::from_millis(1500));
                        println!("Screenshot saved: screenshot_{}.png", generate_timestamp());
                    }
                    "processes" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        let agent_id = session.current_agent.as_ref().unwrap();
                        list_processes(agent_id, &processes);
                    }
                    "kill" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        if args.is_empty() {
                            println!("Usage: kill <pid>");
                        } else if let Ok(pid) = args[0].parse::<u32>() {
                            println!("Terminating process with PID {}...", pid);
                            std::thread::sleep(std::time::Duration::from_millis(500));
                            println!("Process terminated successfully");
                        } else {
                            println!("Invalid PID: {}", args[0]);
                        }
                    }
                    "keylog" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        let action = if args.is_empty() { "status" } else { args[0] };
                        match action {
                            "start" => {
                                println!("Keylogger started on agent");
                            }
                            "stop" => {
                                println!("Keylogger stopped on agent");
                            }
                            "dump" => {
                                println!("Keylogger data:");
                                println!("---------------------");
                                println!("[12:05:32] User opened terminal");
                                println!("[12:05:45] whoami");
                                println!("[12:06:12] ls -la /home");
                                println!("[12:07:03] cd /etc");
                                println!("[12:07:15] cat shadow");
                                println!("---------------------");
                            }
                            "status" => {
                                println!("Keylogger status: RUNNING");
                                println!("Captured keystrokes: 156");
                                println!("Running since: 2025-08-01 11:45:22");
                            }
                            _ => {
                                println!("Unknown keylogger command: {}", action);
                                println!("Available commands: start, stop, dump, status");
                            }
                        }
                    }
                    "info" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        let agent_id = session.current_agent.as_ref().unwrap();
                        display_agent_info(agent_id);
                    }
                    "clear" => {
                        // ANSI escape code to clear the screen
                        print!("\x1B[2J\x1B[1;1H");
                    }
                    "sleep" => {
                        if session.current_agent.is_none() {
                            println!("No agent selected. Use 'use <agent_id>' first.");
                            continue;
                        }

                        if args.is_empty() {
                            println!("Usage: sleep <seconds>");
                        } else if let Ok(seconds) = args[0].parse::<u32>() {
                            println!("Setting agent sleep time to {} seconds", seconds);
                        } else {
                            println!("Invalid sleep time: {}", args[0]);
                        }
                    }
                    "exit" | "quit" => {
                        println!("Shutting down C2 server...");
                        break;
                    }
                    _ => {
                        println!("Unknown command: '{}'", command);
                        println!("Type 'help' for a list of available commands");
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                println!("Ctrl-C received. Use 'exit' to quit.");
            }
            Err(ReadlineError::Eof) => {
                println!("Ctrl-D received. Exiting.");
                break;
            }
            Err(err) => {
                println!("Error reading line: {:?}", err);
                break;
            }
        }
        std::io::stdout().flush().unwrap();
    }

    rl.save_history(history_file)
}

// Display ASCII art banner at startup
fn display_ascii_art() {
    println!("\x1B[31m");
    println!(
        " █     █░ ██▀███   ▄▄▄       ██▓▄▄▄█████▓ ██░ ██  ███▄ ▄███▓ ▄▄▄       ██▀███   ██ ▄█▀▓█████ ▓█████▄ "
    );
    println!(
        "▓█░ █ ░█░▓██ ▒ ██▒▒████▄    ▓██▒▓  ██▒ ▓▒▓██░ ██▒▓██▒▀█▀ ██▒▒████▄    ▓██ ▒ ██▒ ██▄█▒ ▓█   ▀ ▒██▀ ██▌"
    );
    println!(
        "▒█░ █ ░█ ▓██ ░▄█ ▒▒██  ▀█▄  ▒██▒▒ ▓██░ ▒░▒██▀▀██░▓██    ▓██░▒██  ▀█▄  ▓██ ░▄█ ▒▓███▄░ ▒███   ░██   █▌"
    );
    println!(
        "░█░ █ ░█ ▒██▀▀█▄  ░██▄▄▄▄██ ░██░░ ▓██▓ ░ ░▓█ ░██ ▒██    ▒██ ░██▄▄▄▄██ ▒██▀▀█▄  ▓██ █▄ ▒▓█  ▄ ░▓█▄   ▌"
    );
    println!(
        "░░██▒██▓ ░██▓ ▒██▒ ▓█   ▓██▒░██░  ▒██▒ ░ ░▓█▒░██▓▒██▒   ░██▒ ▓█   ▓██▒░██▓ ▒██▒▒██▒ █▄░▒████▒░▒████▓ "
    );
    println!(
        "░ ▓░▒ ▒  ░ ▒▓ ░▒▓░ ▒▒   ▓▒█░░▓    ▒ ░░    ▒ ░░▒░▒░ ▒░   ░  ░ ▒▒   ▓▒█░░ ▒▓ ░▒▓░▒ ▒▒ ▓▒░░ ▒░ ░ ▒▒▓  ▒ "
    );
    println!(
        "  ▒ ░ ░    ░▒ ░ ▒░  ▒   ▒▒ ░ ▒ ░    ░     ▒ ░▒░ ░░  ░      ░  ▒   ▒▒ ░  ░▒ ░ ▒░░ ░▒ ▒░ ░ ░  ░ ░ ▒  ▒ "
    );
    println!(
        "  ░   ░    ░░   ░   ░   ▒    ▒ ░  ░       ░  ░░ ░░      ░     ░   ▒     ░░   ░ ░ ░░ ░    ░    ░ ░  ░ "
    );
    println!(
        "    ░       ░           ░  ░ ░            ░  ░  ░       ░         ░  ░   ░     ░  ░      ░  ░   ░    "
    );
    println!("\x1B[0m");
    println!("                    \x1B[1m\x1B[34m[ WraithMarked C2 Server v1.5.2 ]\x1B[0m");
    println!("                       \x1B[32mAuthor: CyberStackPro\x1B[0m");
    println!();
    println!("Type 'help' for a list of available commands");
    println!();
}

// Display help information
fn display_help() {
    println!("\n\x1B[1m=== WraithMarked C2 Server Commands ===\x1B[0m");

    let mut table = Table::new();
    table
        .load_preset(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Command")
                .add_attribute(Attribute::Bold)
                .fg(Color::Green),
            Cell::new("Description").add_attribute(Attribute::Bold),
        ]);

    table.add_row(vec![Cell::new("help"), Cell::new("Display this help menu")]);
    table.add_row(vec![
        Cell::new("agents"),
        Cell::new("List all active agents"),
    ]);
    table.add_row(vec![
        Cell::new("use <agent_id>"),
        Cell::new("Select an agent to control"),
    ]);
    table.add_row(vec![
        Cell::new("back"),
        Cell::new("Return to main console from agent context"),
    ]);
    table.add_row(vec![
        Cell::new("info"),
        Cell::new("Display detailed information about the selected agent"),
    ]);
    table.add_row(vec![
        Cell::new("ls [path]"),
        Cell::new("List files in the specified path on the agent"),
    ]);
    table.add_row(vec![
        Cell::new("cd <path>"),
        Cell::new("Change directory on the agent"),
    ]);
    table.add_row(vec![
        Cell::new("shell <command>"),
        Cell::new("Execute shell command on the agent"),
    ]);
    table.add_row(vec![
        Cell::new("download <file> [path]"),
        Cell::new("Download file from agent to local machine"),
    ]);
    table.add_row(vec![
        Cell::new("upload <file> <path>"),
        Cell::new("Upload file from local machine to agent"),
    ]);
    table.add_row(vec![
        Cell::new("screenshot"),
        Cell::new("Capture screenshot from agent"),
    ]);
    table.add_row(vec![
        Cell::new("processes"),
        Cell::new("List running processes on agent"),
    ]);
    table.add_row(vec![
        Cell::new("kill <pid>"),
        Cell::new("Terminate process on agent"),
    ]);
    table.add_row(vec![
        Cell::new("keylog [cmd]"),
        Cell::new("Control keylogger (start, stop, dump, status)"),
    ]);
    table.add_row(vec![
        Cell::new("sleep <seconds>"),
        Cell::new("Set agent sleep time between check-ins"),
    ]);
    table.add_row(vec![
        Cell::new("clear"),
        Cell::new("Clear the terminal screen"),
    ]);
    table.add_row(vec![
        Cell::new("exit/quit"),
        Cell::new("Exit the C2 server"),
    ]);

    println!("{table}\n");
}

// Helper function to generate a timestamp
fn generate_timestamp() -> String {
    let now = SystemTime::now();
    let timestamp = now
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    timestamp.to_string()
}

// Helper function to generate some dummy agent data for the table
fn generate_sample_agents() -> Vec<Agent> {
    vec![
        Agent {
            id: "ae1f5-4c2b-8a9d-5f3e4".to_string(),
            last_seen: "2025-08-02 12:00:00".to_string(),
            os: "Windows 11 Pro".to_string(),
            user: "john.doe".to_string(),
            hostname: "CORP-WS01".to_string(),
            ip_address: "192.168.1.105".to_string(),
            privileges: "Administrator".to_string(),
            version: "1.5.2".to_string(),
            active_directory: true,
            sleep: 60,
        },
        Agent {
            id: "b9c8d-1a2e-3b4f-6c7d8".to_string(),
            last_seen: "2025-08-02 12:05:30".to_string(),
            os: "macOS Monterey".to_string(),
            user: "jane.smith".to_string(),
            hostname: "Janes-MacBook-Pro".to_string(),
            ip_address: "192.168.1.120".to_string(),
            privileges: "User".to_string(),
            version: "1.5.2".to_string(),
            active_directory: false,
            sleep: 30,
        },
        Agent {
            id: "d8f7e-2b1a-9c3d-5e4f6".to_string(),
            last_seen: "2025-08-02 12:10:45".to_string(),
            os: "Ubuntu 22.04 LTS".to_string(),
            user: "admin".to_string(),
            hostname: "srv-ubuntu-01".to_string(),
            ip_address: "192.168.1.50".to_string(),
            privileges: "root".to_string(),
            version: "1.5.2".to_string(),
            active_directory: false,
            sleep: 45,
        },
        Agent {
            id: "7a6b5-3c4d-2e1f-9g8h".to_string(),
            last_seen: "2025-08-02 12:15:22".to_string(),
            os: "Windows Server 2022".to_string(),
            user: "system".to_string(),
            hostname: "DC01".to_string(),
            ip_address: "192.168.1.10".to_string(),
            privileges: "SYSTEM".to_string(),
            version: "1.5.1".to_string(),
            active_directory: true,
            sleep: 120,
        },
        Agent {
            id: "5e4f3-2d1c-7b8a-9h0g".to_string(),
            last_seen: "2025-08-02 12:20:18".to_string(),
            os: "Debian 11".to_string(),
            user: "webadmin".to_string(),
            hostname: "web-srv-01".to_string(),
            ip_address: "192.168.1.80".to_string(),
            privileges: "www-data".to_string(),
            version: "1.5.0".to_string(),
            active_directory: false,
            sleep: 90,
        },
    ]
}

// Helper function to create simulated filesystems for agents
fn generate_sample_filesystems() -> HashMap<String, HashMap<String, Vec<File>>> {
    let mut filesystems = HashMap::new();

    // Windows filesystem for agent ae1f5-4c2b-8a9d-5f3e4
    let mut windows_fs = HashMap::new();

    windows_fs.insert(
        "/home".to_string(),
        vec![
            File {
                name: "Documents".to_string(),
                size: 0,
                created: "2025-06-15 10:30:22".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "Downloads".to_string(),
                size: 0,
                created: "2025-06-15 10:30:22".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "Desktop".to_string(),
                size: 0,
                created: "2025-06-15 10:30:22".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "user_credentials.txt".to_string(),
                size: 1024,
                created: "2025-07-30 14:22:15".to_string(),
                content_type: "text".to_string(),
            },
        ],
    );

    windows_fs.insert(
        "/home/Documents".to_string(),
        vec![
            File {
                name: "Project_Plans.docx".to_string(),
                size: 5242880,
                created: "2025-07-15 09:12:33".to_string(),
                content_type: "document".to_string(),
            },
            File {
                name: "Financial_Report_2025.xlsx".to_string(),
                size: 2097152,
                created: "2025-07-20 13:45:01".to_string(),
                content_type: "spreadsheet".to_string(),
            },
            File {
                name: "passwords.txt".to_string(),
                size: 2048,
                created: "2025-07-25 10:18:22".to_string(),
                content_type: "text".to_string(),
            },
        ],
    );

    windows_fs.insert(
        "/home/Desktop".to_string(),
        vec![
            File {
                name: "screenshot_meeting.png".to_string(),
                size: 1048576,
                created: "2025-08-01 15:30:00".to_string(),
                content_type: "image".to_string(),
            },
            File {
                name: "vpn_config.ovpn".to_string(),
                size: 4096,
                created: "2025-07-28 09:15:42".to_string(),
                content_type: "config".to_string(),
            },
        ],
    );

    // macOS filesystem for agent b9c8d-1a2e-3b4f-6c7d8
    let mut macos_fs = HashMap::new();

    macos_fs.insert(
        "/home".to_string(),
        vec![
            File {
                name: "Documents".to_string(),
                size: 0,
                created: "2025-05-10 08:15:30".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "Downloads".to_string(),
                size: 0,
                created: "2025-05-10 08:15:30".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "Pictures".to_string(),
                size: 0,
                created: "2025-05-10 08:15:30".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: ".ssh".to_string(),
                size: 0,
                created: "2025-05-10 08:15:30".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: ".bash_history".to_string(),
                size: 8192,
                created: "2025-08-01 09:45:12".to_string(),
                content_type: "text".to_string(),
            },
        ],
    );

    macos_fs.insert(
        "/home/.ssh".to_string(),
        vec![
            File {
                name: "id_rsa".to_string(),
                size: 1675,
                created: "2025-06-22 11:42:18".to_string(),
                content_type: "key".to_string(),
            },
            File {
                name: "id_rsa.pub".to_string(),
                size: 450,
                created: "2025-06-22 11:42:18".to_string(),
                content_type: "key".to_string(),
            },
            File {
                name: "known_hosts".to_string(),
                size: 3072,
                created: "2025-07-30 16:22:05".to_string(),
                content_type: "text".to_string(),
            },
        ],
    );

    // Linux filesystem for agent d8f7e-2b1a-9c3d-5e4f6
    let mut linux_fs = HashMap::new();

    linux_fs.insert(
        "/home".to_string(),
        vec![
            File {
                name: "admin".to_string(),
                size: 0,
                created: "2025-01-15 14:20:00".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "var".to_string(),
                size: 0,
                created: "2025-01-15 14:20:00".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "etc".to_string(),
                size: 0,
                created: "2025-01-15 14:20:00".to_string(),
                content_type: "directory".to_string(),
            },
        ],
    );

    linux_fs.insert(
        "/home/admin".to_string(),
        vec![
            File {
                name: "script.sh".to_string(),
                size: 2048,
                created: "2025-07-15 13:24:55".to_string(),
                content_type: "script".to_string(),
            },
            File {
                name: ".bash_history".to_string(),
                size: 10240,
                created: "2025-08-01 18:30:42".to_string(),
                content_type: "text".to_string(),
            },
            File {
                name: "backup.tar.gz".to_string(),
                size: 104857600,
                created: "2025-07-28 02:15:00".to_string(),
                content_type: "archive".to_string(),
            },
        ],
    );

    linux_fs.insert(
        "/home/etc".to_string(),
        vec![
            File {
                name: "passwd".to_string(),
                size: 4096,
                created: "2025-07-15 13:24:55".to_string(),
                content_type: "text".to_string(),
            },
            File {
                name: "shadow".to_string(),
                size: 3072,
                created: "2025-07-15 13:24:55".to_string(),
                content_type: "text".to_string(),
            },
            File {
                name: "hosts".to_string(),
                size: 1024,
                created: "2025-07-15 13:24:55".to_string(),
                content_type: "text".to_string(),
            },
        ],
    );

    // Add more filesystems for the other agents
    filesystems.insert("ae1f5-4c2b-8a9d-5f3e4".to_string(), windows_fs);
    filesystems.insert("b9c8d-1a2e-3b4f-6c7d8".to_string(), macos_fs);
    filesystems.insert("d8f7e-2b1a-9c3d-5e4f6".to_string(), linux_fs);

    // Server filesystem
    let mut server_fs = HashMap::new();
    server_fs.insert(
        "/home".to_string(),
        vec![
            File {
                name: "www".to_string(),
                size: 0,
                created: "2025-01-10 10:00:00".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "logs".to_string(),
                size: 0,
                created: "2025-01-10 10:00:00".to_string(),
                content_type: "directory".to_string(),
            },
            File {
                name: "database".to_string(),
                size: 0,
                created: "2025-01-10 10:00:00".to_string(),
                content_type: "directory".to_string(),
            },
        ],
    );

    server_fs.insert(
        "/home/www".to_string(),
        vec![
            File {
                name: "index.php".to_string(),
                size: 4096,
                created: "2025-06-10 11:22:33".to_string(),
                content_type: "script".to_string(),
            },
            File {
                name: "config.php".to_string(),
                size: 2048,
                created: "2025-06-10 11:22:40".to_string(),
                content_type: "script".to_string(),
            },
            File {
                name: "db_credentials.ini".to_string(),
                size: 1024,
                created: "2025-06-10 11:23:15".to_string(),
                content_type: "config".to_string(),
            },
        ],
    );

    server_fs.insert(
        "/home/database".to_string(),
        vec![
            File {
                name: "customers.db".to_string(),
                size: 52428800,
                created: "2025-07-30 23:15:00".to_string(),
                content_type: "database".to_string(),
            },
            File {
                name: "employees.db".to_string(),
                size: 10485760,
                created: "2025-07-30 23:15:10".to_string(),
                content_type: "database".to_string(),
            },
            File {
                name: "products.db".to_string(),
                size: 31457280,
                created: "2025-07-30 23:15:20".to_string(),
                content_type: "database".to_string(),
            },
        ],
    );

    filesystems.insert("7a6b5-3c4d-2e1f-9g8h".to_string(), server_fs);

    filesystems
}

// Helper function to generate sample processes for agents
fn generate_sample_processes() -> HashMap<String, Vec<Process>> {
    let mut processes = HashMap::new();

    // Windows processes
    processes.insert(
        "ae1f5-4c2b-8a9d-5f3e4".to_string(),
        vec![
            Process {
                pid: 4,
                name: "System".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.1,
                memory: 0.5,
            },
            Process {
                pid: 388,
                name: "svchost.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.2,
                memory: 1.2,
            },
            Process {
                pid: 672,
                name: "lsass.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.0,
                memory: 0.8,
            },
            Process {
                pid: 704,
                name: "winlogon.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.0,
                memory: 0.6,
            },
            Process {
                pid: 1028,
                name: "explorer.exe".to_string(),
                user: "john.doe".to_string(),
                cpu: 0.5,
                memory: 5.2,
            },
            Process {
                pid: 1256,
                name: "chrome.exe".to_string(),
                user: "john.doe".to_string(),
                cpu: 2.3,
                memory: 15.4,
            },
            Process {
                pid: 1892,
                name: "outlook.exe".to_string(),
                user: "john.doe".to_string(),
                cpu: 1.1,
                memory: 8.7,
            },
            Process {
                pid: 2240,
                name: "Word.exe".to_string(),
                user: "john.doe".to_string(),
                cpu: 0.4,
                memory: 6.2,
            },
            Process {
                pid: 3452,
                name: "agent.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.1,
                memory: 1.8,
            },
        ],
    );

    // macOS processes
    processes.insert(
        "b9c8d-1a2e-3b4f-6c7d8".to_string(),
        vec![
            Process {
                pid: 1,
                name: "launchd".to_string(),
                user: "root".to_string(),
                cpu: 0.1,
                memory: 0.2,
            },
            Process {
                pid: 145,
                name: "WindowServer".to_string(),
                user: "root".to_string(),
                cpu: 0.8,
                memory: 1.5,
            },
            Process {
                pid: 256,
                name: "Finder".to_string(),
                user: "jane.smith".to_string(),
                cpu: 0.3,
                memory: 2.1,
            },
            Process {
                pid: 380,
                name: "Safari".to_string(),
                user: "jane.smith".to_string(),
                cpu: 3.2,
                memory: 12.8,
            },
            Process {
                pid: 423,
                name: "Terminal".to_string(),
                user: "jane.smith".to_string(),
                cpu: 0.2,
                memory: 1.3,
            },
            Process {
                pid: 589,
                name: "iTunes".to_string(),
                user: "jane.smith".to_string(),
                cpu: 1.5,
                memory: 8.2,
            },
            Process {
                pid: 1024,
                name: "agent".to_string(),
                user: "root".to_string(),
                cpu: 0.1,
                memory: 0.5,
            },
        ],
    );

    // Linux processes
    processes.insert(
        "d8f7e-2b1a-9c3d-5e4f6".to_string(),
        vec![
            Process {
                pid: 1,
                name: "systemd".to_string(),
                user: "root".to_string(),
                cpu: 0.0,
                memory: 0.2,
            },
            Process {
                pid: 432,
                name: "sshd".to_string(),
                user: "root".to_string(),
                cpu: 0.0,
                memory: 0.3,
            },
            Process {
                pid: 658,
                name: "nginx".to_string(),
                user: "www-data".to_string(),
                cpu: 0.2,
                memory: 1.8,
            },
            Process {
                pid: 789,
                name: "apache2".to_string(),
                user: "www-data".to_string(),
                cpu: 0.3,
                memory: 2.5,
            },
            Process {
                pid: 1024,
                name: "mysqld".to_string(),
                user: "mysql".to_string(),
                cpu: 1.2,
                memory: 12.6,
            },
            Process {
                pid: 1122,
                name: "php-fpm".to_string(),
                user: "www-data".to_string(),
                cpu: 0.4,
                memory: 3.2,
            },
            Process {
                pid: 2048,
                name: "agent".to_string(),
                user: "root".to_string(),
                cpu: 0.1,
                memory: 0.5,
            },
            Process {
                pid: 2345,
                name: "bash".to_string(),
                user: "admin".to_string(),
                cpu: 0.0,
                memory: 0.2,
            },
        ],
    );

    // Server processes
    processes.insert(
        "7a6b5-3c4d-2e1f-9g8h".to_string(),
        vec![
            Process {
                pid: 4,
                name: "System".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.1,
                memory: 0.4,
            },
            Process {
                pid: 328,
                name: "svchost.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.3,
                memory: 2.1,
            },
            Process {
                pid: 452,
                name: "lsass.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.0,
                memory: 1.0,
            },
            Process {
                pid: 632,
                name: "sqlservr.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 2.5,
                memory: 25.3,
            },
            Process {
                pid: 780,
                name: "w3wp.exe".to_string(),
                user: "NETWORK SERVICE".to_string(),
                cpu: 1.2,
                memory: 8.6,
            },
            Process {
                pid: 1256,
                name: "dns.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.1,
                memory: 2.4,
            },
            Process {
                pid: 1580,
                name: "agent.exe".to_string(),
                user: "SYSTEM".to_string(),
                cpu: 0.1,
                memory: 1.7,
            },
        ],
    );

    processes
}

// Helper function to create the "pretty table" from agent data
fn create_agents_table(agents: Vec<Agent>) -> Table {
    let mut table = Table::new();
    table
        .load_preset(UTF8_ROUND_CORNERS)
        .set_content_arrangement(ContentArrangement::Dynamic)
        .set_header(vec![
            Cell::new("Agent ID").add_attribute(Attribute::Bold),
            Cell::new("Last Seen").add_attribute(Attribute::Bold),
            Cell::new("OS").add_attribute(Attribute::Bold),
            Cell::new("User").add_attribute(Attribute::Bold),
            Cell::new("Hostname").add_attribute(Attribute::Bold),
            Cell::new("IP Address").add_attribute(Attribute::Bold),
            Cell::new("Privileges").add_attribute(Attribute::Bold),
        ]);

    for agent in agents {
        // Color privileges based on level (red for high, yellow for medium, green for low)
        let priv_cell = match agent.privileges.to_lowercase().as_str() {
            "system" | "root" | "administrator" => Cell::new(agent.privileges).fg(Color::Red),
            "admin" => Cell::new(agent.privileges).fg(Color::Yellow),
            _ => Cell::new(agent.privileges).fg(Color::Green),
        };

        table.add_row(vec![
            Cell::new(agent.id).fg(Color::Green),
            Cell::new(agent.last_seen).fg(Color::Cyan),
            Cell::new(agent.os),
            Cell::new(agent.user),
            Cell::new(agent.hostname),
            Cell::new(agent.ip_address),
            priv_cell,
        ]);
    }
    table
}

// Function to list files in a simulated filesystem
fn list_files(
    agent_id: &str,
    path: &str,
    filesystems: &HashMap<String, HashMap<String, Vec<File>>>,
) {
    if let Some(filesystem) = filesystems.get(agent_id) {
        if let Some(files) = filesystem.get(path) {
            let mut table = Table::new();
            table
                .load_preset(UTF8_ROUND_CORNERS)
                .set_content_arrangement(ContentArrangement::Dynamic)
                .set_header(vec![
                    Cell::new("Name").add_attribute(Attribute::Bold),
                    Cell::new("Size").add_attribute(Attribute::Bold),
                    Cell::new("Created").add_attribute(Attribute::Bold),
                    Cell::new("Type").add_attribute(Attribute::Bold),
                ]);

            for file in files {
                let name_cell = if file.content_type == "directory" {
                    Cell::new(format!("{}/", file.name)).fg(Color::Blue)
                } else {
                    Cell::new(&file.name)
                };

                let size_str = if file.content_type == "directory" {
                    "-".to_string()
                } else if file.size < 1024 {
                    format!("{} B", file.size)
                } else if file.size < 1024 * 1024 {
                    format!("{:.1} KB", file.size as f64 / 1024.0)
                } else if file.size < 1024 * 1024 * 1024 {
                    format!("{:.1} MB", file.size as f64 / (1024.0 * 1024.0))
                } else {
                    format!("{:.1} GB", file.size as f64 / (1024.0 * 1024.0 * 1024.0))
                };

                table.add_row(vec![
                    name_cell,
                    Cell::new(size_str),
                    Cell::new(&file.created),
                    Cell::new(&file.content_type),
                ]);
            }

            println!("Listing contents of: {}", path);
            println!("{table}");
        } else {
            println!("Directory not found: {}", path);
        }
    } else {
        println!("Filesystem not found for agent: {}", agent_id);
    }
}

// Function to list processes in a simulated system
fn list_processes(agent_id: &str, processes: &HashMap<String, Vec<Process>>) {
    if let Some(process_list) = processes.get(agent_id) {
        let mut table = Table::new();
        table
            .load_preset(UTF8_ROUND_CORNERS)
            .set_content_arrangement(ContentArrangement::Dynamic)
            .set_header(vec![
                Cell::new("PID").add_attribute(Attribute::Bold),
                Cell::new("Name").add_attribute(Attribute::Bold),
                Cell::new("User").add_attribute(Attribute::Bold),
                Cell::new("CPU %").add_attribute(Attribute::Bold),
                Cell::new("Memory %").add_attribute(Attribute::Bold),
            ]);

        for process in process_list {
            // Highlight the agent process in green
            let name_cell = if process.name.contains("agent") {
                Cell::new(&process.name).fg(Color::Green)
            } else {
                Cell::new(&process.name)
            };

            // Highlight high-privilege processes
            let user_cell = match process.user.to_lowercase().as_str() {
                "system" | "root" => Cell::new(&process.user).fg(Color::Red),
                _ => Cell::new(&process.user),
            };

            table.add_row(vec![
                Cell::new(process.pid.to_string()),
                name_cell,
                user_cell,
                Cell::new(format!("{:.1}", process.cpu)),
                Cell::new(format!("{:.1}", process.memory)),
            ]);
        }

        println!("Running processes on agent:");
        println!("{table}");
    } else {
        println!("No process information available for agent: {}", agent_id);
    }
}

// Function to display detailed agent info
fn display_agent_info(agent_id: &str) {
    let agents = generate_sample_agents();
    let agent = agents.iter().find(|a| a.id == agent_id);

    match agent {
        Some(agent) => {
            println!("\n\x1B[1m=== Agent Information ===\x1B[0m");
            println!("ID:               \x1B[32m{}\x1B[0m", agent.id);
            println!("Last Seen:        \x1B[36m{}\x1B[0m", agent.last_seen);
            println!("OS:               {}", agent.os);
            println!("User:             {}", agent.user);
            println!("Hostname:         {}", agent.hostname);
            println!("IP Address:       {}", agent.ip_address);
            println!("Privileges:       {}", agent.privileges);
            println!("Agent Version:    {}", agent.version);
            println!("Active Directory: {}", agent.active_directory);
            println!("Sleep Time:       {} seconds", agent.sleep);
            println!();
        }
        None => {
            println!("Agent not found: {}", agent_id);
        }
    }
}

// Simulate executing a shell command
fn simulate_shell_command(command: &str) {
    println!("\x1B[33m[*] Executing command: {}\x1B[0m", command);
    println!("\x1B[2mCommand output:\x1B[0m");

    if command.starts_with("whoami") {
        println!("john.doe");
    } else if command.starts_with("hostname") {
        println!("CORP-WS01");
    } else if command.starts_with("ipconfig") || command.starts_with("ifconfig") {
        println!("Ethernet adapter Local Area Connection:");
        println!("   Connection-specific DNS Suffix  . : corp.local");
        println!("   IPv4 Address. . . . . . . . . . . : 192.168.1.105");
        println!("   Subnet Mask . . . . . . . . . . . : 255.255.255.0");
        println!("   Default Gateway . . . . . . . . . : 192.168.1.1");
    } else if command.starts_with("net user") || command.starts_with("cat /etc/passwd") {
        println!("admin:x:1000:1000:Administrator:/home/admin:/bin/bash");
        println!("john.doe:x:1001:1001:John Doe:/home/john.doe:/bin/bash");
        println!("jane.smith:x:1002:1002:Jane Smith:/home/jane.smith:/bin/bash");
        println!("www-data:x:33:33:www-data:/var/www:/usr/sbin/nologin");
    } else if command.starts_with("ps") || command.starts_with("tasklist") {
        println!("  PID TTY          TIME CMD");
        println!("    1 ?        00:00:05 systemd");
        println!("  432 ?        00:00:01 sshd");
        println!("  658 ?        00:00:03 nginx");
        println!(" 1024 ?        00:00:18 mysqld");
        println!(" 2048 ?        00:00:00 agent");
    } else if command.contains("passwd") || command.contains("shadow") {
        println!("root:$6$xyz123$aB7cD8eFgH:18936:0:99999:7:::");
        println!("admin:$6$abc456$iJ9kL0mN1o:18936:0:99999:7:::");
        println!("john.doe:$6$def789$pQ2rS3tU4v:18936:0:99999:7:::");
    } else {
        println!("Command executed successfully, but produced no output.");
    }

    println!("\x1B[33m[*] Command execution completed\x1B[0m");
}
