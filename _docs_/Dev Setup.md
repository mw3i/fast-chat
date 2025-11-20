---
title: Dev Guide
source: ai
---

This guide will help you set up the development environment for Fast Chat Launcher.

# Prerequisites

The project requires:
- **Rust** + Cargo
- **Node.js** + npm
- **Tauri CLI**

# macOS Setup

## 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Install Tauri CLI

```bash
cargo install tauri-cli
```

## 3. Install Node.js

```bash
brew install node
```

## 4. Install Frontend Dependencies

```bash
cd frontend && npm install
```

# Linux Setup (Debian/Ubuntu)

## 1. Install Rust

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

## 2. Install Tauri CLI

```bash
cargo install tauri-cli
```

## 3. Install Node.js

```bash
sudo apt install nodejs npm
```

## 4. Install Frontend Dependencies

```bash
cd frontend && npm install
```

# Running the Development Server

Once dependencies are installed, you can run the development server:

```bash
make desktop
```

Or manually:

```bash
cd desktop && cargo tauri dev
```

This will:
- Start the frontend dev server (Vite)
- Build and run the Tauri app
- Open the app window

# Building for Production

To build the production bundle:

```bash
cd desktop && cargo tauri build
```

The built application will be in `desktop/target/release/bundle/`.

# Optional: Ollama for Local Models

If you want to use local models via Ollama:

1. Install Ollama from [ollama.ai](https://ollama.ai)
2. Pull a model: `ollama pull gemma2:2b` (though I think it'll just automatically download but not sure)
3. Open app -> Settings -> Set model provider to Ollama
