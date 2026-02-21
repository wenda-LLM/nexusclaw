<p align="center">
  <img src="zeroclaw.png" alt="NexusClaw" width="200" />
</p>

<h1 align="center">NexusClaw 🦀</h1>

<p align="center">
  <strong>Enterprise Multi-Tenant AI Assistant Infrastructure</strong><br>
  Forked from <a href="https://github.com/zeroclaw-labs/zeroclaw">ZeroClaw</a> with enterprise enhancements
</p>

<p align="center">
  <a href="LICENSE"><img src="https://img.shields.io/badge/license-Apache--2.0-blue.svg" alt="License: Apache-2.0" /></a>
  <a href="https://github.com/wenda-LLM/nexusclaw"><img src="https://img.shields.io/github/stars/wenda-LLM/nexusclaw" alt="Stars" /></a>
  <a href="https://github.com/wenda-LLM/nexusclaw/fork"><img src="https://img.shields.io/badge/Fork-this%20project-blue.svg" alt="Fork" /></a>
</p>

## What is NexusClaw?

NexusClaw is a fork of [ZeroClaw](https://github.com/zeroclaw-labs/zeroclaw) with additional enterprise features for multi-tenant deployments. It inherits all ZeroClaw's core capabilities:

- 🏎️ **Lean Runtime**: <5MB RAM, Rust single binary
- ⚡ **Fast Cold Starts**: <10ms startup time  
- 🌍 **Portable**: ARM/x86/RISC-V support
- 🔒 **Secure by Default**: Pairing, sandboxing, workspace scoping

## NexusClaw Enterprise Features

### Multi-Tenant Architecture

- 🏢 **Tenant Management**: Full tenant isolation with `Tenant` and `TenantStore`
- 👥 **User Management**: Role-based access control with `User`, `UserRole`, `UserSession`
- 👥 **Group Management**: Team organization with `Group`, `GroupMember`, `GroupStore`

### Enterprise Security

- 🔐 **Vault System**: Secure secret storage at group level (`GroupVaultStore`, `VaultStore`)
- 🔑 **Key Rotation**: Automated credential rotation (`key_rotation.rs`)
- 📊 **Capability Ceiling**: Fine-grained permission controls per user level
- 🛡️ **Agent Credentials**: Secure credential management for agent containers

### Container & Relay

- 🖥️ **Container Management**: Agent container lifecycle with resource limits (`ContainerManager`)
- 🔄 **Relay Server**: Infrastructure for distributed agent deployments

## Architecture

```
┌─────────────────────────────────────────────┐
│              NexusClaw Core                 │
│  (Inherited from ZeroClaw - all features)   │
├─────────────────────────────────────────────┤
│         Enterprise Extensions               │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐       │
│  │ Tenant  │ │  Vault  │ │Container│       │
│  │ Manager │ │  System │ │ Manager │       │
│  └─────────┘ └─────────┘ └─────────┘       │
│  ┌─────────┐ ┌─────────┐ ┌─────────┐       │
│  │  User   │ │  Group  │ │  Relay  │       │
│  │ Manager │ │ Manager │ │ Server  │       │
│  └─────────┘ └─────────┘ └─────────┘       │
└─────────────────────────────────────────────┘
```

## Quick Start

```bash
# Clone and build (same as ZeroClaw)
git clone https://github.com/wenda-LLM/nexusclaw.git
cd nexusclaw
cargo build --release

# Run (same commands as ZeroClaw)
./target/release/zeroclaw onboard --api-key sk-... --provider openrouter
./target/release/zeroclaw agent -m "Hello!"
```

## What's Different from ZeroClaw?

| Feature | ZeroClaw | NexusClaw |
|---------|----------|-----------|
| Single tenant | ✅ | ✅ |
| Multi-tenant | ❌ | ✅ |
| Group vault | ❌ | ✅ |
| User management | ❌ | ✅ |
| Container management | ❌ | ✅ |
| Key rotation | ❌ | ✅ |
| Relay server | ❌ | ✅ |

## Upstream Sync

NexusClaw is actively synced with upstream ZeroClaw. To sync:

```bash
git fetch upstream
git merge upstream/main
```

## License

Apache-2.0 (same as ZeroClaw)

---

**NexusClaw** — Enterprise-grade AI Assistant Infrastructure 🦀