# WraithMarked - Implementation Status & Roadmap

## 📋 Current Implementation Status

### ✅ AGENT (wraithmarked_agent)

#### Implemented Features
- [x] **Keystroke Tracking**
  - [x] Global keyboard event capture (rdev)
  - [x] Mouse events (clicks, movement, scroll)
  - [x] Statistics tracking (total counts, rate calculations)
  - [x] Periodic data aggregation (per-minute stats)
  - [x] JSON file export every 5 minutes
  - [x] Graceful shutdown with Ctrl+C

- [x] **Active Window Monitoring**
  - [x] Track focused application/window
  - [x] Capture window title, process name, PID, path
  - [x] Browser URL extraction (Windows/macOS)
  - [x] Duration tracking per window
  - [x] Window change detection

- [x] **Architecture & Code Quality**
  - [x] Thread-safe design (Arc<Mutex>)
  - [x] Modular structure (models, services, utils)
  - [x] Clean separation of concerns
  - [x] Error handling with logging

#### Not Implemented (Agent)
- [ ] **C2 Communication**
  - [ ] HTTP/HTTPS beacon client
  - [ ] Command receiving from C2
  - [ ] Command execution engine
  - [ ] Result transmission to C2
  - [ ] Configurable beacon interval
  - [ ] Connection retry logic with exponential backoff

- [ ] **Local Data Storage**
  - [ ] SQLite database integration
  - [ ] Store activity data when offline (3-5 days retention)
  - [ ] Auto-cleanup old data
  - [ ] Queue commands when disconnected

- [ ] **Command Handlers**
  - [ ] Shell command execution
  - [ ] File system operations (ls, cd, download, upload)
  - [ ] Process management (list, kill)
  - [ ] Screenshot capture
  - [ ] Activity data retrieval

- [ ] **Configuration**
  - [ ] C2 server address configuration
  - [ ] Beacon interval configuration
  - [ ] Feature toggles (keylogger on/off, etc.)
  - [ ] Stealth settings

- [ ] **Persistence**
  - [ ] Autostart mechanism
  - [ ] Service installation
  - [ ] Self-respawning capability

---

### ⚠️ C2 SERVER (wraithmarked_c2)

#### Implemented Features
- [x] **Mock CLI Interface**
  - [x] Interactive shell with rustyline
  - [x] Pretty table output (comfy-table)
  - [x] Command parsing
  - [x] All commands (agents, use, shell, ls, etc.) with fake data
  - [x] Session management concept

- [x] **Utilities**
  - [x] TCP connection helpers with timeout
  - [x] File download functions (blocking/non-blocking)
  - [x] Basic Axum hello world routes

#### Not Implemented (C2)
- [ ] **Core Server Infrastructure**
  - [ ] Agent registration endpoint
  - [ ] Agent beacon/check-in endpoint
  - [ ] Command queue system
  - [ ] Result collection endpoint
  - [ ] WebSocket support (future)

- [ ] **Agent Management**
  - [ ] Agent data structure (id, os, user, ip, status, last_seen)
  - [ ] In-memory agent storage (HashMap)
  - [ ] Agent lifecycle tracking
  - [ ] Heartbeat monitoring

- [ ] **Command & Control Logic**
  - [ ] Command creation and queuing
  - [ ] Command ID generation
  - [ ] Result correlation (command_id → result)
  - [ ] Timeout handling

- [ ] **CLI Integration**
  - [ ] Connect CLI to real backend (remove mock data)
  - [ ] Route commands from CLI to agents via API
  - [ ] Display real agent data
  - [ ] Stream real command results

- [ ] **Data Persistence**
  - [ ] Database integration (SQLite/PostgreSQL)
  - [ ] Store agents, commands, results
  - [ ] Activity log archiving
  - [ ] Session history

- [ ] **Security**
  - [ ] HTTPS/TLS support
  - [ ] Authentication/authorization
  - [ ] Encrypted payloads
  - [ ] Certificate pinning

---

## 🎯 MVP Roadmap (Phase 1)

### Goal: Basic C2 ↔ Agent Communication

**Priority Order:**
1. ✅ Create implementation checklist (this file)
2. ⏳ **Build C2 HTTP Server (Axum)**
   - Agent registration endpoint
   - Beacon endpoint (get pending commands)
   - Result submission endpoint
   - In-memory agent storage

3. ⏳ **Build Agent HTTP Client**
   - Initial registration with C2
   - Periodic beacon loop
   - Command fetching
   - Result submission
   - SQLite for offline storage

4. ⏳ **Implement Basic Commands**
   - `shell` - Execute shell commands
   - `sysinfo` - Return system information
   - `ping` - Connection test

5. ⏳ **Connect CLI to Real Backend**
   - Replace mock data with HTTP API calls
   - Display real agents
   - Send commands to C2 API
   - Show real results

---

## 🏗️ Architecture Design

### Communication Flow (HTTP-based MVP)

```
┌─────────────┐          ┌─────────────┐          ┌─────────────┐
│   CLI       │          │   C2 Server │          │   Agent     │
│  (Terminal) │          │   (Axum)    │          │  (Target)   │
└─────────────┘          └─────────────┘          └─────────────┘
      │                         │                         │
      │  1. User types cmd      │                         │
      │───────────────────────▶ │                         │
      │  POST /api/command      │                         │
      │                         │                         │
      │                         │  2. Agent beacons       │
      │                         │ ◀───────────────────────│
      │                         │  GET /api/beacon        │
      │                         │                         │
      │                         │  3. C2 returns pending  │
      │                         │     commands            │
      │                         │ ────────────────────────▶
      │                         │                         │
      │                         │  4. Agent executes      │
      │                         │                         │
      │                         │  5. Agent sends result  │
      │                         │ ◀───────────────────────│
      │                         │  POST /api/result       │
      │                         │                         │
      │  6. CLI polls for result│                         │
      │ ◀───────────────────────│                         │
      │  GET /api/result/:id    │                         │
```

### Message Format (JSON)

```rust
// Agent → C2: Registration
{
    "type": "register",
    "agent_id": "uuid-v4",
    "hostname": "target-pc",
    "os": "Linux",
    "os_version": "Ubuntu 22.04",
    "user": "admin",
    "ip": "192.168.1.100",
    "privileges": "root",
    "version": "0.1.0"
}

// C2 → Agent: Commands (from beacon response)
{
    "commands": [
        {
            "id": "cmd-uuid",
            "type": "shell",
            "payload": "whoami"
        }
    ]
}

// Agent → C2: Results
{
    "agent_id": "uuid-v4",
    "command_id": "cmd-uuid",
    "success": true,
    "output": "root\n",
    "timestamp": "2025-12-04T..."
}
```

---

## 📦 Technology Stack

### Agent
- **Language:** Rust 2021
- **Async Runtime:** Tokio
- **HTTP Client:** reqwest
- **Input Capture:** rdev
- **Window Tracking:** x-win
- **Database:** SQLite (rusqlite) - **TO ADD**
- **Serialization:** serde, serde_json

### C2 Server
- **Language:** Rust 2024
- **Web Framework:** Axum 0.8
- **Async Runtime:** Tokio
- **CLI:** rustyline, comfy-table
- **Database:** SQLite (future: PostgreSQL)
- **Serialization:** serde, serde_json

---

## 🚀 Next Steps

### Where to Start?
**Answer: C2 Server First**

**Why?**
1. Agent needs something to connect to
2. Easier to test endpoints with `curl` or Postman
3. Define protocol/API first = clear contract
4. Can use mock agent initially (curl commands)

### Learning Path
1. **Day 1-2:** Build C2 Axum endpoints (registration, beacon, results)
2. **Day 3-4:** Add agent HTTP client + beacon loop
3. **Day 5-6:** Implement command execution on agent
4. **Day 7:** Connect CLI to real C2 backend
5. **Day 8+:** Add SQLite, more commands, polish

---

## 🎓 Concepts to Learn

### Axum Fundamentals
- [ ] Routing (`Router::new()`, `.route()`)
- [ ] Handlers (async functions)
- [ ] Extractors (`Json`, `Path`, `Query`, `State`)
- [ ] Shared state (`Arc<Mutex<T>>`)
- [ ] Middleware (logging, cors)
- [ ] Error handling

### Async Rust
- [ ] `async`/`await` syntax
- [ ] Tokio runtime
- [ ] `tokio::spawn` for background tasks
- [ ] Channels (mpsc) for communication
- [ ] `Arc<Mutex<T>>` for shared state

### HTTP Client (reqwest)
- [ ] GET/POST requests
- [ ] JSON serialization/deserialization
- [ ] Headers, authentication
- [ ] Connection pooling
- [ ] Retry logic

---

## 🔐 Security Considerations (Future)

- [ ] HTTPS only (no plain HTTP in production)
- [ ] Certificate pinning
- [ ] Payload encryption (AES-GCM)
- [ ] Agent authentication tokens
- [ ] Rate limiting on C2
- [ ] Obfuscation (strings, function names)
- [ ] Anti-debugging techniques
- [ ] Process injection (advanced)

---

## 📝 Notes

- This is an **educational project** for authorized pentesting only
- Never deploy on systems you don't own/have permission to test
- Focus on clean code and learning Rust properly
- Iterate: MVP → Features → Polish
- Test incrementally, don't build everything at once

---

**Last Updated:** 2025-12-04
**Current Phase:** Pre-MVP (Setting up communication)
**Next Milestone:** Basic C2-Agent HTTP communication working
