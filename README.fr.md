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
</p>

<p align="center">
  ⚡️ <strong>Multi-tenant Web UI</strong> · 🔐 <strong>Enterprise Vault</strong> · 👥 <strong>Team Management</strong>
</p>

<p align="center">
  🌐 <strong>Language:</strong> <a href="README.md">English</a> · <a href="README.zh-CN.md">简体中文</a>
</p>

> This page is under construction. Please see the [English README](README.md) for full documentation.

### About NexusClaw

NexusClaw is an enterprise fork of [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) with:

- **Multi-Tenant Web UI:** Built-in web interface for agent management
- **Enterprise Vault:** Secure secret storage for team credentials
- **Team Management:** User and group management with RBAC
- **Container Management:** Agent container lifecycle with resource limits
- **Key Rotation:** Automated credential rotation
- **Relay Server:** Distributed agent deployment infrastructure

### Quick Start

```bash
# Web UI mode (recommended)
zeroclaw server --port 8080

# CLI mode (same as ZeroClaw)
cargo build --release
./target/release/zeroclaw onboard --api-key sk-...
./target/release/zeroclaw agent -m "Hello!"
```

### License

Apache-2.0

---

**NexusClaw** — Enterprise Multi-Tenant AI Assistant Platform 🦀