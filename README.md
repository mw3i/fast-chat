---
title: Fast Chat Launcher
source: human + ai
---

![Rabbit Cerberus](desktop/app-icon.png)

# What is this

Spotlight-style quick launcher for AI chat. Supports locally (Ollama, LM Studio) and externally (OpenAI, Google) hosted models.

![](_docs_/.assets/1.gif)

# Why is this

Sometimes I just need to talk to an LLM really quickly, and Raycast is paywalled.

# How to use

**No Warranty:** This software is provided "AS IS" without warranty of any kind. Use at your own risk.

1. Download (coming soon): [Mac]() | [Linux]() | [Windows]()
2. Run app
3. `ctrl+space` or whatever you set the keyboard shortcut to
4. Chat with LLM

> **macOS Note:** The app is not code signed (this is an open source project). On first launch, macOS will likely block it. To run:
> 
> 1. Right-click the app → "Open"
> 2. Click "Open" in the security dialog
>

## Set Custom Keybinding

Open app -> Settings -> Set custom key binding

![](_docs_/.assets/2.gif)

## Using a local llm

1. Download and run [Ollama](https://ollama.com)
2. Open app -> Settings -> Set Model provider = Ollama
3. Chat with LLM

![](_docs_/.assets/3.gif)

# Dev

## Requirements

- Rust + Cargo
  - Tauri CLI (`cargo install tauri-cli`)
- Node.js + npm

## Build from source

- clone repo
- initialize dev environment (see `_docs_/Dev Setup.md`)
- run `make desktop` (or: `cd desktop && cargo tauri dev`)

# How it works

## Architecture

- Tauri desktop app (Rust backend + Svelte frontend)
- Rust backend handles:
  - LLM API calls (Ollama, OpenAI, Google, LM Studio, Custom)
  - Conversation history storage (JSON files)
  - Settings management (config.json)

> Eventually we'll probably want to bring in a Python FastAPI server for complex agent workflows

# Contribute

Don't screw it up.

# Notes

- 99% of the code was written with AI, guided via Cursor.

# ⚠️ Disclaimer

**API Usage & Costs:** This software makes API calls to third-party services (OpenAI, Google, etc.) that may charge based on usage. You are responsible for monitoring and managing your API usage and costs. The authors are not responsible for any charges incurred from using this software.

**No Warranty:** This software is provided "AS IS" without warranty of any kind. Use at your own risk.

## UNTESTED!

This hasn't been tested yet for:

- [ ] model provider = google api 
- [ ] model provider = lm studio version
- [ ] windows
- [ ] linux

So your mileage may vary.
