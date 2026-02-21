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
  <a href="NOTICE"><img src="https://img.shields.io/badge/contributors-27+-green.svg" alt="Contributeurs" /></a>
  <a href="https://buymeacoffee.com/argenistherose"><img src="https://img.shields.io/badge/Buy%20Me%20a%20Coffee-Donate-yellow.svg?style=flat&logo=buy-me-a-coffee" alt="Offrez-moi un café" /></a>
  <a href="https://x.com/zeroclawlabs?s=21"><img src="https://img.shields.io/badge/X-%40zeroclawlabs-000000?style=flat&logo=x&logoColor=white" alt="X : @zeroclawlabs" /></a>
  <a href="https://zeroclawlabs.cn/group.jpg"><img src="https://img.shields.io/badge/WeChat-Group-B7D7A8?logo=wechat&logoColor=white" alt="WeChat Group" /></a>
  <a href="https://www.xiaohongshu.com/user/profile/67cbfc43000000000d008307?xsec_token=AB73VnYnGNx5y36EtnnZfGmAmS-6Wzv8WMuGpfwfkg6Yc%3D&xsec_source=pc_search"><img src="https://img.shields.io/badge/Xiaohongshu-Official-FF2442?style=flat" alt="Xiaohongshu : Officiel" /></a>
  <a href="https://t.me/zeroclawlabs"><img src="https://img.shields.io/badge/Telegram-%40zeroclawlabs-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram : @zeroclawlabs" /></a>
  <a href="https://t.me/zeroclawlabs_cn"><img src="https://img.shields.io/badge/Telegram%20CN-%40zeroclawlabs__cn-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram CN : @zeroclawlabs_cn" /></a>
  <a href="https://t.me/zeroclawlabs_ru"><img src="https://img.shields.io/badge/Telegram%20RU-%40zeroclawlabs__ru-26A5E4?style=flat&logo=telegram&logoColor=white" alt="Telegram RU : @zeroclawlabs_ru" /></a>
  <a href="https://www.reddit.com/r/zeroclawlabs/"><img src="https://img.shields.io/badge/Reddit-r%2Fzeroclawlabs-FF4500?style=flat&logo=reddit&logoColor=white" alt="Reddit : r/zeroclawlabs" /></a>
  <a href="https://github.com/wenda-LLM/nexusclaw"><img src="https://img.shields.io/github/stars/wenda-LLM/nexusclaw" alt="Stars" /></a>
  <a href="https://github.com/wenda-LLM/nexusclaw/fork"><img src="https://img.shields.io/badge/Fork-blue.svg" alt="Fork" /></a>
</p>
<p align="center">
Construit par des étudiants et membres des communautés Harvard, MIT et Sundai.Club.
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