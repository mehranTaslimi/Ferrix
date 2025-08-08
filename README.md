<p align="center">
  <img src="./assets/logo.png" alt="Ferrix Logo" width="128"/>
</p>

<h1 align="center">Ferrix 🦀 ⬇️</h1>

<p align="center">
  <b>A blazing-fast, crash-resistant, and extensible download manager built with Rust + Tauri.</b>
</p>

<p align="center">
  <a href="https://github.com/mehranTaslimi/Ferrix/releases">
    <img src="https://img.shields.io/github/v/release/mehranTaslimi/Ferrix" alt="Release">
  </a>
  <a href="https://github.com/mehranTaslimi/Ferrix/stargazers">
    <img src="https://img.shields.io/github/stars/mehranTaslimi/Ferrix" alt="Stars">
  </a>
  <a href="https://github.com/mehranTaslimi/Ferrix/issues">
    <img src="https://img.shields.io/github/issues/mehranTaslimi/Ferrix" alt="Issues">
  </a>
  <a href="LICENSE">
    <img src="https://img.shields.io/github/license/mehranTaslimi/Ferrix" alt="License">
  </a>
</p>

---

## 🚀 Features

- ⚡ **Parallel & Chunked Downloads** – maximize bandwidth with multi-threaded chunks
- 🔄 **Resumable Downloads** – pick up exactly where you left off, even after crashes
- 🌍 **Proxy Support** – HTTP/SOCKS5 proxies for privacy and flexibility
- 🛡 **Crash Safety + Data Integrity** – database-backed progress tracking
- ♻ **Automatic Retry** – smart reconnection with exponential backoff
- 🔌 **Modular Architecture** – future-ready for BitTorrent, plugins, and extensions
- 🖥 **Cross-Platform UI** – minimal, modern interface built with Tauri + TailwindCSS

---

## 📦 Installation

Download the latest version of Ferrix for your operating system from the **[Releases](https://github.com/mehranTaslimi/Ferrix/releases)** page.

1. Go to the [Releases page](https://github.com/mehranTaslimi/Ferrix/releases).
2. Find the latest release at the top.
3. Download the installer or archive for your platform:
   - **Windows** – `.msi` or `.exe`
   - **macOS** – `.dmg`
   - **Linux** – `.AppImage` or `.deb`
4. Install and run Ferrix 🚀

---

## 🛠 Build from Source

```bash
# Clone the repo
git clone https://github.com/mehranTaslimi/Ferrix.git
cd Ferrix

# Install frontend dependencies
pnpm install

# Run development mode
pnpm dev:all
```

---

## 📸 Screenshots

<p align="center">
  <img src="./assets/screenshot.png" width="700" alt="Ferrix UI Screenshot"/>
</p>

---

## 🏗 Architecture

Ferrix is built with a modern, modular architecture:

- **Rust** + **Tokio** – high-performance, memory-safe backend with powerful async networking
- **Tauri** – secure & lightweight cross-platform desktop framework
- **Next.js** + **ShadCN UI** – fast, component-driven frontend with beautiful and consistent design
- **SQLite** – reliable, crash-safe database for download progress tracking
