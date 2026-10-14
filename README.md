# Clipflow

A lightweight Rust daemon that watches the system clipboard and proposes contextual actions via native desktop notifications. When you copy something, Clipflow classifies the content and offers a relevant action — no GUI, no manual triggering, just copy and act.

## What it does

- **Copy a URL** → "Open clean link", "Send to Telegram"
- **Copy a YouTube URL** → "Download audio", "Open in mpv"
- **Copy an IP address** → "Ping it", "Check location"
- **Copy a file path** → "Open in file manager", "Copy contents"
- **Copy a port number** → "Check what's running"
- **Copy a Cargo dependency** → "Open docs.rs"
- **Copy a Docker image** → "Generate compose service"
- **Copy JSON** → "Prettify", "Validate"
- **Copy a regex** → "Test against clipboard history"
- **Copy an API endpoint** → "Generate curl command"
- **Copy a cron expression** → "Explain next 5 runtimes"
- **Copy a PID** → "Kill process"
- **Copy a math expression** → "Show result"
- **Copy a systemd service** → "Status", "Logs", "Restart"


## Philosophy

- **Zero GUI.** Actions are presented as native notifications or egui with clickable buttons.
- **Offline-first.** Most handlers work without internet. External calls (Telegram, docs.rs) are optional.
- **Pluggable.** Each content type is a handler module. Enable/disable via config.
- **Homeserver-aware.** Optional integrations (Telegram bot, BeaverHabits, etc.) when Tailscale is up.
- **Rust-native.**

## Architecture

- `daemon`: Polls clipboard, classifies content, fires notifications.
- `classifier`: Regex-based content type detection.
- `handlers`: One module per content type. Each defines match logic and available actions.
- `notifier`: `notify-rust` integration. Actions map to D-Bus notification buttons.
- `config`: TOML file enabling/disabling handlers and setting homeserver endpoints.