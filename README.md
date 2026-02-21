<p align="center">
  <img src="zeroclaw.png" alt="NexusClaw" width="200" />
</p>

<h1 align="center">NexusClaw 🦀</h1>

<p align="center">
  <strong>Enterprise Multi-Tenant AI Assistant Platform</strong><br>
  Web-based agent management with multi-tenant support — Built on ZeroClaw
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License: Apache-2.0" /></a>
  <a href="https://github.com/wenda-LLM/nexusclaw"><img src="https://img.shields.io/github/stars/wenda-LLM/nexusclaw" alt="Stars" /></a>
  <a href="https://github.com/wenda-LLM/nexusclaw/fork"><img src="https://img.shields.io/badge/Fork-blue.svg" alt="Fork" /></a>
</p>

<p align="center">
  ⚡️ <strong>Multi-tenant Web UI</strong> · 🔐 <strong>Enterprise Vault</strong> · 👥 <strong>Team Management</strong>
</p>

<p align="center">
  🌐 <strong>Languages:</strong> <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a>
</p>

<p align="center">
  <a href="#quick-start">Getting Started</a> |
  <a href="bootstrap.sh">One-Click Setup</a> |
  <a href="docs/README.md">Docs Hub</a> |
  <a href="docs/SUMMARY.md">Docs TOC</a>
</p>

<p align="center">
  <strong>Quick Routes:</strong>
  <a href="docs/reference/README.md">Reference</a> ·
  <a href="docs/operations/README.md">Operations</a> ·
  <a href="docs/troubleshooting.md">Troubleshoot</a> ·
  <a href="docs/security/README.md">Security</a> ·
  <a href="docs/hardware/README.md">Hardware</a> ·
  <a href="docs/contributing/README.md">Contribute</a>
</p>

<p align="center">
  <strong>Enterprise AI Assistant Platform with Web-based Management</strong><br />
  Multi-tenant architecture with built-in web UI for team collaboration
</p>

<p align="center">
  NexusClaw is an <strong>enterprise fork</strong> of ZeroClaw — a multi-tenant AI assistant platform with a built-in web-based management interface.
</p>

<p align="center"><code>Multi-tenant · Web UI · Enterprise Vault · Team Management · Built on ZeroClaw</code></p>

### 📢 About NexusClaw

NexusClaw is a fork of [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) focused on enterprise use cases with multi-tenant architecture and web-based UI access.

| Feature | Description |
|---------|-------------|
| **Multi-Tenant** | Full tenant isolation with tenant, user, and group management |
| **Web UI** | Built-in web interface for agent management (included in binary) |
| **Enterprise Vault** | Secure secret storage at group level |
| **Container Management** | Agent container lifecycle with resource limits |
| **Key Rotation** | Automated credential rotation |
| **Relay Server** | Distributed agent deployment infrastructure |

### Why Teams Choose NexusClaw

- **Web-based Access:** Browser-based UI for managing agents — no CLI required for daily operations
- **Multi-Tenant Ready:** Built-in tenant/user/group management for enterprise deployments
- **Secure by Default:** Inherits ZeroClaw's security features (pairing, sandboxing, workspace scoping)
- **Lean & Fast:** Rust single binary, <5MB RAM, <10ms cold start
- **Fully Inherits ZeroClaw:** All ZeroClaw features (providers, channels, tools, memory) work out of the box

### ✨ Features (Inherited from ZeroClaw)

- 🏎️ **Lean Runtime by Default:** Common CLI and status workflows run in a few-megabyte memory envelope on release builds.
- 💰 **Cost-Efficient Deployment:** Designed for low-cost boards and small cloud instances without heavyweight runtime dependencies.
- ⚡ **Fast Cold Starts:** Single-binary Rust runtime keeps command and daemon startup near-instant for daily operations.
- 🌍 **Portable Architecture:** One binary-first workflow across ARM, x86, and RISC-V with swappable providers/channels/tools.

### NexusClaw New Features

- 🌐 **Web-based Management UI:** Built-in web interface (port 42617) for agent chat, configuration, logs, memory, tools, and integrations
- 🏢 **Multi-Tenant Support:** Tenant isolation with full user and group management
- 🔐 **Enterprise Vault:** Secure secret storage for team credentials
- 🖥️ **Container Management:** Agent container lifecycle with resource limits
- 🔑 **Key Rotation:** Automated credential rotation for enhanced security
- 🔄 **Relay Server:** Infrastructure for distributed agent deployments
- 📊 **Capability Ceiling:** Fine-grained permission controls per user level

## Quick Start

### Web UI Mode (Recommended)

```bash
# Start the web server (default: 127.0.0.1:42617)
zeroclaw server

# Or start with specific port
zeroclaw server --port 8080

# Access the web UI at http://127.0.0.1:42617
```

### CLI Mode (Same as ZeroClaw)

```bash
# Clone and build
git clone https://github.com/wenda-LLM/nexusclaw.git
cd nexusclaw
cargo build --release

# Quick setup
./target/release/zeroclaw onboard --api-key sk-... --provider openrouter

# Chat
./target/release/zeroclaw agent -m "Hello!"

# Interactive mode
./target/release/zeroclaw agent

# Start gateway
./target/release/zeroclaw gateway

# Start daemon
./target/release/zeroclaw daemon
```

## Web UI Features

The built-in web UI provides:

- **Agent Chat:** Interactive chat with the AI agent
- **Configuration:** Manage providers, models, and settings
- **Logs:** Real-time log viewing and filtering
- **Memory:** View and manage agent memory
- **Tools:** Browse available tools
- **Integrations:** Manage channel integrations
- **Cost:** View usage and cost metrics
- **Cron Jobs:** Schedule and manage tasks
- **Doctor:** System diagnostics

## Architecture

```
┌─────────────────────────────────────────────────────────────┐
│                     NexusClaw Platform                       │
├─────────────────────────────────────────────────────────────┤
│  ┌─────────────────┐    ┌─────────────────┐                 │
│  │   Web Server    │    │   CLI / Gateway │                 │
│  │  (axum-based)   │    │   (ZeroClaw)    │                 │
│  └────────┬────────┘    └────────┬────────┘                 │
│           │                      │                          │
│  ┌────────▼──────────────────────▼────────┐                │
│  │           Multi-Tenant Layer            │                │
│  │  Tenant | User | Group | Vault | Container│              │
│  └─────────────────────────────────────────┘                │
│                         │                                    │
│  ┌──────────────────────▼──────────────────┐                │
│  │          ZeroClaw Core Runtime           │                │
│  │  Provider | Channel | Tool | Memory     │                │
│  └─────────────────────────────────────────┘                │
└─────────────────────────────────────────────────────────────┘
```

## Configuration

NexusClaw uses the same config format as ZeroClaw (`~/.zeroclaw/config.toml`):

```toml
api_key = "sk-..."
default_provider = "openrouter"
default_model = "anthropic/claude-sonnet-4-6"

[server]
host = "127.0.0.1"
port = 42617

[gateway]
port = 42617
host = "127.0.0.1"

[memory]
backend = "sqlite"
```

## Multi-Tenant Management

### Tenant Operations

```bash
# List tenants
zeroclaw tenant list

# Create tenant
zeroclaw tenant create --name mycompany

# Switch tenant context
zeroclaw tenant use mycompany
```

### User Management

```bash
# Add user to tenant
zeroclaw user add --name john --role member

# List users
zeroclaw user list

# Update user role
zeroclaw user update john --role admin
```

### Group & Vault

```bash
# Create group
zeroclaw group create --name engineering

# Add secrets to vault
zeroclaw vault set OPENAI_API_KEY sk-...

# List vault secrets
zeroclaw vault list
```

## Security

NexusClaw inherits all ZeroClaw security features plus:

- Tenant isolation with role-based access control
- Group-level vault for team credentials
- Agent credential management
- Key rotation support

## Development

```bash
cargo build              # Dev build
cargo build --release    # Release build
cargo test               # Run tests
cargo fmt                # Format
cargo clippy             # Lint
```

## License

Apache-2.0 (same as ZeroClaw)

---

**NexusClaw** — Enterprise Multi-Tenant AI Assistant Platform 🦀