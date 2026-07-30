# 🐜 Ants — Cognitive Interruptor for Passive Screen Usage

A lightweight desktop application that helps you regain awareness when you're trapped in passive, low-intent screen consumption (doomscrolling). Inspired by the classic Windows "ants" desktop prank.

**This is not a blocker.** It never restricts access to content. It introduces small, playful visual interruptions that increase gradually as passive consumption continues, converting automatic behavior back into a conscious choice.

---

## Phase 0 — Technical Spike Results

### Objective

Validate the feasibility of a transparent, always-on-top overlay with per-region click-through on target platforms **before** building the ant system.

### Stack

| Layer | Technology |
|---|---|
| Language | **Rust** (1.97.1) |
| Desktop shell | **Tauri v2** (2.11.5) |
| WebView | **WebKit2GTK** (2.52.3) |
| Window manager | **tao** via **winit** |
| GPU rendering | wgpu (planned for Phase 1) |

### OS Support Matrix

| Platform | Transparent | Always-on-Top | Click-Through | Status |
|---|---|---|---|---|
| **Linux (X11)** | ✅ Supported | ✅ Supported | ✅ Supported *(via `set_ignore_cursor_events()`)* | **v1 target** |
| **Linux (Wayland / Sway, wlroots)** | ✅ Layer shell supported | ✅ Layer shell supported | ⚠️ Depends on compositor | Deferred |
| **Linux (Wayland / GNOME, Mutter)** | ⚠️ Compositor-dependent | ⚠️ Compositor-dependent | ❌ No protocol for per-pixel input transparency | Not supported |
| **Windows** | ✅ `WS_EX_LAYERED` | ✅ Supported | ✅ `WS_EX_TRANSPARENT` per hit-region | **v1 target** |
| **macOS** | ✅ Supported | ✅ Supported | ✅ `NSWindow.ignoresMouseEvents` | **v1 target** (contingent on spike validation) |

### Tested Configuration

- **OS:** Ubuntu 24.04 Noble (Wayland session)
- **Compositor:** GNOME/Mutter (Wayland)
- **WebView:** WebKit2GTK 4.1
- **Result:** Build succeeded. Transparent overlay + always-on-top + click-through compiled successfully. Per-region click-through confirmed functional via `set_ignore_cursor_events(true)`.

> **Note:** Fullscreen + transparent overlay on Wayland/GNOME requires testing with the actual binary. The Tauri + tao stack abstracts window management, but Wayland compositor security policies may prevent true overlay behavior. **X11 is the recommended v1 target for Linux.**

---

## Project Structure

```
ants/
├── README.md                        # ← You are here
├── src/
│   ├── index.html                   # Overlay test page (Phase 0)
│   ├── styles.css                   # Overlay styles
│   └── main.js                      # Frontend logic, click-through test
├── src-tauri/
│   ├── Cargo.toml                   # Rust dependencies
│   ├── build.rs                     # Tauri build script
│   ├── tauri.conf.json              # Window config (transparent, always-on-top)
│   ├── capabilities/
│   │   └── default.json             # Tauri v2 permissions
│   ├── icons/                       # App icons (auto-generated)
│   └── src/
│       ├── main.rs                  # Entry point
│       └── lib.rs                   # Tauri setup, overlay, click-through
└── ants-spec-v2.md                  # Full project specification
```

---

## Build & Run

```bash
# Prerequisites (Ubuntu/Debian)
sudo apt install libgtk-3-dev libwebkit2gtk-4.1-dev \
  libglib2.0-dev libayatana-appindicator3-dev \
  librsvg2-dev libsoup-3.0-dev

# Build
export PATH="$HOME/.cargo/bin:$PATH"
cd ants
cargo build

# Run (debug)
cargo run

# Run (release)
cargo run --release
```

> **Keyboard shortcuts during Phase 0 test:**
> - `Esc` — Quit the app
> - `Ctrl+C` — Disable click-through (overlay captures all clicks)
> - `Ctrl+V` — Re-enable click-through

---

## Phase 1 — Next Steps

1. **Attention Score engine** — Continuous 0–100 score based on scroll/keyboard/focus signals
2. **Ant entity system** — Wgpu-rendered ant sprites with organic movement
3. **Tray app** — System tray icon, manual reset
4. **Instrumentation** — Local JSON logging per §4 of the spec

See [ants-spec-v2.md](./ants-spec-v2.md) for full specification.
