# WraithMarked Agent

> _“Crao: The one who deciphers chaos. The silent ghost in your code.”_

![Insert Architecture Diagram or Project Logo Here](assets/banner_placeholder.png)

---

⚠️ **DISCLAIMER: FOR EDUCATIONAL AND ETHICAL SECURITY RESEARCH ONLY**

This project is a proof-of-concept agent designed for **authorized penetration testing**, cybersecurity research, and defensive tool development.
**Do not deploy or run this on any system you do not own or have explicit, written consent to monitor.** Unauthorized usage is strictly prohibited and may be illegal.

---

## 🧠 Project Overview

**WraithMarked Agent** is a minimal, stealth-first, cross-platform activity monitoring agent (RAT-style PoC) built with Rust.
It focuses on clean system visibility while remaining resource-efficient, leveraging low-level system hooks for insight collection.

Built to explore real-world techniques used in red teaming, adversary simulation, and digital forensics tooling.

---

## 🔍 Core Features (In Progress)

- **Keystroke Logging** – Captures global keyboard input
- **Active Window Monitoring** – Tracks currently focused applications
- **Autostart** – Adds persistence via OS startup routines
- **Stealth Execution** – No visible window, tray icon, or desktop footprint
- **C2 Communication (Planned)** – Sends encrypted telemetry to remote endpoint
- **Live Screen Preview (Planned)** – Captures screen for remote viewing
- **Remote Shell & File Access (Planned)** – Execute commands & inspect filesystem

---

## 📦 Tech Stack

- **Rust** – Performance, safety, concurrency
- **Libraries:**

  - `rdev` – Low-level global input capture
  - `x-win` – Active window info
  - `chrono`, `serde`, `reqwest`, `tokio`, `log` – Logging, async, and telemetry
  - `ctrlc` – Graceful shutdown handling

---

## 🚧 Development Roadmap

### Phase 1 – Core Agent

- [x] Input (keyboard + clicks)
- [x] Active window capture
- [ ] Local data storage
- [ ] Autostart + persistence
- [ ] Logging + error capture
- [ ] Minimal network transmission

### Phase 2 – Remote Capabilities

- [ ] Secure Command & Control (C2) connection
- [ ] Remote shell / terminal execution
- [ ] File system browsing
- [ ] Live screen snapshots

---

## ⚙️ Getting Started

### Prerequisites

- [Rust](https://www.rust-lang.org/tools/install)

### Installation

```sh
git clone https://github.com/cyberstackpro/wraithmarked-agent.git
cd wraithmarked-agent
cargo build --release
```
