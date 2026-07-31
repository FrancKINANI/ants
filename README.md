# 🐜 Ants — Cognitive Interruptor for Passive Screen Usage

A lightweight desktop application that helps you regain awareness when you're trapped in passive, low-intent screen consumption (doomscrolling). Inspired by the classic Windows "ants" desktop prank.

**This is not a blocker.** It never restricts access to content. It introduces small, playful visual interruptions that increase gradually as passive consumption continues, converting automatic behavior back into a conscious choice.

---

## ✨ Current Status — Phase 1 MVP Complete

**Phase 1 MVP is fully implemented and functional.** The application includes:

- ✅ **Attention Score Engine** — Continuous 0–100 score based on scroll/keyboard/focus signals
- ✅ **Ant Entity System** — Canvas-rendered procedural ants with organic movement AI
- ✅ **System Tray** — Tray icon with menu for manual reset and status
- ✅ **Local Instrumentation** — JSON session logging for hypothesis validation
- ✅ **Sound System** — Web Audio API for ant dismissal feedback
- ✅ **Dynamic Click-Through** — Overlay lets clicks pass through except on ants

---

## 🛠️ Tech Stack

| Layer | Technology |
|---|---|
| Language | **Rust** (backend) + **JavaScript** (frontend) |
| Desktop shell | **Tauri v2** |
| WebView | **WebKit2GTK** (Linux), **WebView2** (Windows), **WKWebView** (macOS) |
| Rendering | **HTML5 Canvas** (procedural ant rendering) |
| Configuration | **TOML** (`~/.ants/config.toml`) |
| Logging | **JSON Lines** (`~/.ants/sessions.jsonl`) |

---

## 🖥️ Platform Support

| Platform | Transparent | Always-on-Top | Click-Through | Status |
|---|---|---|---|---|
| **Linux (X11)** | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Tested |
| **Linux (Wayland)** | ⚠️ Compositor-dependent | ⚠️ Compositor-dependent | ⚠️ Limited | ⚠️ Best effort |
| **Windows** | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Target |
| **macOS** | ✅ Supported | ✅ Supported | ✅ Supported | ✅ Target |

> **Note:** Linux X11 is the recommended platform for full functionality. Wayland support depends on compositor capabilities.

---

## 📁 Project Structure

```
ants/
├── README.md                        # ← You are here
├── CONTRIBUTING.md                  # Contribution guidelines
├── ARCHITECTURE.md                  # Technical architecture overview
├── src/
│   ├── index.html                   # Main overlay interface
│   ├── styles.css                   # Overlay styles
│   ├── main.js                      # Frontend orchestration & Tauri communication
│   └── ants.js                      # Canvas ant rendering engine
├── src-tauri/
│   ├── Cargo.toml                   # Rust dependencies
│   ├── build.rs                     # Tauri build script
│   ├── tauri.conf.json              # Window & app configuration
│   ├── capabilities/
│   │   └── default.json             # Tauri v2 permissions
│   ├── icons/                       # App icons
│   └── src/
│       ├── main.rs                  # Entry point
│       ├── lib.rs                   # Tauri setup & command handlers
│       ├── score.rs                 # Attention Score engine
│       ├── input.rs                 # Input event parser
│       ├── settings.rs              # Configuration management
│       ├── tray.rs                  # System tray implementation
│       └── instrumentation.rs       # Session logging
└── ants-spec-v2.md                  # Original specification (archived)
```

---

## 🚀 Installation & Usage

### Prerequisites

**Ubuntu/Debian:**
```bash
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev \
  libglib2.0-dev libayatana-appindicator3-dev \
  librsvg2-dev libsoup-3.0-dev
```

**macOS:**
```bash
# Install Rust and dependencies via Homebrew
brew install rust
```

**Windows:**
```bash
# Install Rust from https://rustup.rs/
# Webview2 is included with Windows 10/11
```

### Building

```bash
# Clone the repository
git clone https://github.com/yourusername/ants.git
cd ants

# Build
cargo build

# Run (debug mode)
cargo run

# Run (release mode - optimized)
cargo run --release
```

### Configuration

The app creates a configuration file at `~/.ants/config.toml` on first run:

```toml
[score]
scroll_penalty_per_min = 25.0
keyboard_silence_penalty_per_min = 5.0
passive_window_penalty = 10.0
# ... more settings

[general]
enable_sound = false
enable_local_instrumentation = true
spawn_interval_seconds = 5.0
max_ants = 30
overlay_opacity = 1.0
```

### Keyboard Shortcuts

- **`Esc`** — Quit the application
- **`R`** — Reset attention score and dismiss all ants
- **`H`** — Toggle overlay visibility (click-through when hidden)
- **`D`** — Demo mode: spawn ants regardless of score (for testing)

### Data Files

- **Config:** `~/.ants/config.toml` — Application settings
- **Logs:** `~/.ants/sessions.jsonl` — Session data for analysis

---

## 🧠 How It Works

### Attention Score System

The app continuously calculates an "attention score" (0–100) based on:

- **Passive signals** (decrease score): Continuous scrolling, keyboard silence, passive app windows
- **Active signals** (increase score): Keyboard activity, deliberate clicks, switching to work apps

**Score thresholds:**
- **100–40:** No ants (user is engaged)
- **39–25:** 1 ant spawns (subtle reminder)
- **24–10:** 5 ants on screen (moderate presence)
- **9–0:** Up to 30 ants (full intervention)

### The Ants

Ants are rendered procedurally using HTML5 Canvas with:
- Organic movement AI (random walks, pauses, direction changes)
- Smooth fade-in/fade-out transitions
- Click interaction (splat animation + sound)
- Automatic cleanup when attention score recovers

### Privacy

- **No keystroke logging** — Only event types and rates are tracked
- **No content inspection** — Window titles are only matched against known patterns
- **Local-only processing** — All computation happens on your machine
- **No data transmission** — Session logs are stored locally only

---

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines.

For architecture details, see [ARCHITECTURE.md](./ARCHITECTURE.md).

For ideas on future improvements, see [IMPROVEMENTS.md](./IMPROVEMENTS.md).

---

## 📄 License

MIT

---

## 🙏 Acknowledgments

Inspired by the classic Windows "ants" desktop prank and the concept of gentle friction in behavioral design.
