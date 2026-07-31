# Architecture Documentation

This document provides a technical overview of the Ants application architecture, covering the frontend-backend separation, data flow, and key design decisions.

## 🏗️ High-Level Architecture

Ants follows a **client-server architecture** where:

- **Backend (Rust)** handles business logic, system integration, and state management
- **Frontend (JavaScript)** handles rendering, user interaction, and animation
- **Tauri** provides the bridge between them via IPC (Inter-Process Communication)

```
┌─────────────────────────────────────────────────────────────┐
│                     Desktop Environment                     │
│  (System tray, window management, OS input events)          │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│                    Tauri Runtime (Rust)                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              Tauri Commands (IPC)                     │  │
│  │  • feed_event()  • get_score_snapshot()               │  │
│  │  • reset_score()  • toggle_click_through()            │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                         │                                    │
│  ┌──────────────────────▼───────────────────────────────┐  │
│  │              Business Logic Layer                    │  │
│  │  • ScoreEngine    • Logger    • AntsConfig           │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                         │                                    │
│  ┌──────────────────────▼───────────────────────────────┐  │
│  │              System Integration Layer                 │  │
│  │  • Tray         • Window Manager    • File I/O       │  │
│  └──────────────────────────────────────────────────────┘  │
└──────────────────────┬──────────────────────────────────────┘
                       │
┌──────────────────────▼──────────────────────────────────────┐
│                   WebView (JavaScript)                      │
│  ┌──────────────────────────────────────────────────────┐  │
│  │              AntEngine (Canvas Renderer)              │  │
│  │  • Ant entities  • Animation loop  • Interaction      │  │
│  └──────────────────────┬───────────────────────────────┘  │
│                         │                                    │
│  ┌──────────────────────▼───────────────────────────────┐  │
│  │              Frontend Orchestration                   │  │
│  │  • Score polling  • Event forwarding  • UI updates   │  │
│  └──────────────────────────────────────────────────────┘  │
└──────────────────────────────────────────────────────────────┘
```

## 🔄 Frontend-Backend Communication

### Tauri Commands (Rust → JavaScript)

The backend exposes functions that the frontend can call via `invoke()`:

```rust
// In lib.rs
#[tauri::command]
fn feed_event(state: tauri::State<'_, AppState>, event_type: String, window_title: Option<String>) {
    // Process event
}
```

```javascript
// In main.js
await invoke('feed_event', { event_type: 'scroll', window_title: 'YouTube' });
```

### Tauri Events (JavaScript → Rust)

The backend can emit events that the frontend listens to:

```rust
// In tray.rs
window.emit("ants:reset", ());
```

```javascript
// In main.js
window.__TAURI__.core.listen('ants:reset', () => {
    engine.reset();
});
```

### Shared State

The backend maintains shared state protected by `Mutex` for thread-safe access:

```rust
struct AppState {
    score_engine: Mutex<ScoreEngine>,
    logger: Mutex<Logger>,
}
```

## 🦀 Rust Backend Modules

### `main.rs` — Entry Point

Minimal entry point that delegates to the library:

```rust
fn main() {
    ants_lib::run()
}
```

This pattern allows the library to be tested independently.

### `lib.rs` — Application Core

Responsibilities:
- **Tauri setup** — Window configuration, always-on-top, click-through
- **State management** — Creates and manages `AppState`
- **Command handlers** — Implements Tauri commands as the API surface
- **System tray** — Initializes tray icon and menu

Key commands:
- `feed_event()` — Feeds input events to the score engine
- `get_score_snapshot()` — Returns current attention score
- `reset_score()` — Resets score to 100
- `toggle_click_through()` — Controls window click-through behavior
- `log_ant_spawn/dismiss/user_left()` — Records instrumentation events
- `flush_logger()` — Writes session logs to disk

### `score.rs` — Attention Score Engine

The core business logic that calculates user engagement.

**Key Components:**

```rust
pub struct ScoreEngine {
    config: ScoreConfig,
    score: f64,              // Current score (0-100)
    scroll_count: u64,       // Event counters
    keyboard_count: u64,
    click_count: u64,
    // ... timing and tracking state
}
```

**Algorithm:**
1. **Feed events** — `feed(InputEvent)` records event types and timestamps
2. **Update score** — `update(dt)` applies penalties/recoveries based on time elapsed
3. **Calculate level** — Determines ant count based on score thresholds
4. **Snapshot** — Returns current state for frontend consumption

**Score Dynamics:**
- Passive signals (scroll, silence, passive windows) decrease score
- Active signals (keyboard, clicks, work apps) increase score
- Asymmetric recovery — easier to recover from low scores
- Time-based decay — continuous penalty for sustained passive behavior

### `input.rs` — Input Event Parser

Converts frontend-reported events into Rust types:

```rust
pub enum InputEvent {
    Scroll,
    Keyboard,
    Click,
    WindowFocus(String),
}
```

**Privacy Note:** Only parses event types, never content. The frontend sends event type strings like "scroll", "keyboard", etc., not actual keystrokes.

### `settings.rs` — Configuration Management

Handles TOML configuration file at `~/.ants/config.toml`.

**Structure:**
```rust
pub struct AntsConfig {
    pub score: ScoreConfig,           // Score engine settings
    pub general: GeneralConfig,       // App-wide settings
    pub passive_window_patterns: Vec<String>,  // Passive app detection
    pub active_window_patterns: Vec<String>,   // Active app detection
}
```

**Behavior:**
- Loads config on startup, creates default if missing
- Provides sensible defaults for all settings
- Allows runtime configuration changes (requires restart in v1)

### `tray.rs` — System Tray

Creates and manages the system tray icon and menu.

**Features:**
- Tray icon with current status
- Menu items: Reset, Status, Quit
- Click-to-toggle window visibility
- Emits events to frontend on menu actions

### `instrumentation.rs` — Session Logging

Records session data for hypothesis validation.

**Log Format (JSON Lines):**
```json
{
  "date": "1722472800",
  "session_duration_minutes": 45.3,
  "attention_score_timeline": [{"t": 0, "score": 100}, ...],
  "ants_spawned": 12,
  "ants_dismissed": 8,
  "time_until_user_left_seconds": 34.5
}
```

**Privacy:** Local-only, never transmitted. Used to validate the core hypothesis that gentle interruption reduces passive consumption.

## 🌐 JavaScript Frontend Modules

### `main.js` — Frontend Orchestration

Coordinates between the ant engine and Tauri backend.

**Responsibilities:**
- **Score polling** — Calls `get_score_snapshot()` every second
- **Event forwarding** — Sends scroll/keyboard/click events to backend
- **Tray events** — Listens for reset events from system tray
- **Keyboard shortcuts** — Handles Esc, R, H, D keys
- **Cleanup** — Flushes logs on app close

**Data Flow:**
```javascript
// Poll score every second
engine.onPollScore = async (eng) => {
    const snap = await invoke('get_score_snapshot');
    eng.setLevel(snap.level, snap.score);
};

// Forward input events
document.addEventListener('scroll', () => {
    invoke('feed_event', { event_type: 'scroll' });
});
```

### `ants.js` — Ant Rendering Engine

Pure Canvas-based rendering with procedural ant generation.

**Key Components:**

```javascript
// Ant entity
function createAnt(x, y, dpr) {
    return {
        id, x, y, angle, speed,
        state: 'fadingIn',  // fadingIn, walking, pausing, fadingOut, squashed
        opacity, scale, walkFrame, // ...
    };
}

// Rendering
function drawAnt(ctx, ant, dpr) {
    // Procedural drawing of head, thorax, abdomen, legs
}
```

**Animation System:**
- **Walk cycle** — 6-10 frames per cycle using sine waves for leg movement
- **State machine** — Each ant transitions between states (fade in → walk → pause → fade out)
- **Organic movement** — Random speed variations, pauses, direction changes
- **Interaction** — Click detection with hit testing, splat animation

**Performance:**
- Uses `requestAnimationFrame` for smooth 60fps rendering
- Object pooling could be added for optimization (not needed for current scale)
- Canvas 2D API sufficient for current ant count (max 30)

### `index.html` — UI Structure

Minimal HTML structure:

```html
<div id="overlay">
    <div id="status-bar">Score display</div>
    <canvas id="ant-canvas"></canvas>
    <div id="debug">Debug info</div>
</div>
```

### `styles.css` — Styling

Handles overlay appearance:
- Transparent background
- Full-screen canvas
- Status bar positioning
- Click-through control

## 📊 Data Flow Examples

### Scenario: User Starts Doomscrolling

```
1. User opens YouTube and starts scrolling
   ↓
2. JavaScript detects scroll event
   ↓
3. invoke('feed_event', { event_type: 'scroll', window_title: 'YouTube' })
   ↓
4. input::parse_frontend_event() → InputEvent::Scroll + WindowFocus("YouTube")
   ↓
5. score_engine.feed() — records scroll, detects passive window
   ↓
6. score_engine.update(dt) — applies scroll penalty, passive window penalty
   ↓
7. Score drops from 100 → 35
   ↓
8. Frontend polls: get_score_snapshot() returns { score: 35, level: 'moderate' }
   ↓
9. eng.setLevel('moderate', 35) → target ant count = 1
   ↓
10. Ant spawns with fade-in animation
```

### Scenario: User Clicks an Ant

```
1. User clicks on canvas at (x, y)
   ↓
2. Canvas click handler checks ant hitboxes
   ↓
3. Ant found → transition to 'squashed' state
   ↓
4. Play splat sound (Web Audio API)
   ↓
5. invoke('log_ant_dismiss')
   ↓
6. Logger records dismissal for session log
   ↓
7. Ant fades out after animation
```

### Scenario: User Resets via Tray

```
1. User clicks "Reset" in tray menu
   ↓
2. tray.rs emits "ants:reset" event
   ↓
3. Frontend listens: window.__TAURI__.core.listen('ants:reset')
   ↓
4. engine.reset() — fades out all ants
   ↓
5. invoke('reset_score')
   ↓
6. score_engine.reset() — score back to 100
```

## 🔑 Key Design Decisions

### 1. Rust Backend + JavaScript Frontend

**Rationale:**
- Rust provides system-level access (tray, window management) and performance
- JavaScript excels at Canvas rendering and animation
- Tauri provides a secure, type-safe bridge between them

**Trade-offs:**
- Context switching between languages can be complex
- Debugging跨语言 issues requires understanding both stacks

### 2. Attention Score as Continuous Value

**Rationale:**
- More nuanced than binary "passive/active" detection
- Allows gradual intervention (1 ant → 5 ants → 30 ants)
- Easier to tune and adjust thresholds

**Trade-offs:**
- Requires careful calibration of weights and thresholds
- More complex than simple time-based detection

### 3. Local-Only Instrumentation

**Rationale:**
- Privacy-first design — no data leaves the machine
- User maintains full control over their data
- Simplifies compliance and trust

**Trade-offs:**
- No aggregate analytics for product improvement
- Users must manually export logs for analysis

### 4. Procedural Canvas Rendering

**Rationale:**
- No external assets needed (images, sprites)
- Fully customizable via code
- Scales perfectly to any DPI/resolution

**Trade-offs:**
- More complex than sprite-based rendering
- Performance limited by Canvas 2D API (GPU acceleration not fully utilized)

### 5. Event-Based Architecture

**Rationale:**
- Loose coupling between frontend and backend
- Easy to extend with new event types
- Natural fit for Tauri's command/event system

**Trade-offs:**
- Slight latency compared to direct function calls
- More boilerplate for simple operations

## 🔧 Extension Points

### Adding New Input Signals

1. Add event type to `InputEvent` enum in `score.rs`
2. Update `input::parse_frontend_event()` to handle new type
3. Add weight/penalty logic in `ScoreEngine::feed()` or `update()`
4. Update frontend to detect and forward new events

### Adding New Ant Behaviors

1. Add new state to ant state machine in `ants.js`
2. Implement transition logic in the animation loop
3. Add rendering code for new state in `drawAnt()`
4. Update spawn/despawn logic if needed

### Adding New Configuration Options

1. Add field to appropriate config struct in `settings.rs`
2. Update `Default` implementation with sensible value
3. Use the config value in relevant module
4. Document in README.md

### Adding New Platform Support

1. Update `tauri.conf.json` for platform-specific settings
2. Add platform-specific code with `#[cfg(target_os = "...")]`
3. Test on target platform
4. Update platform support matrix in README

## 🚀 Performance Considerations

### Backend (Rust)

- **Score updates** — Called at ~60fps, but minimal computation
- **Event parsing** — O(1) string matching, negligible overhead
- **File I/O** — Config loaded once, logs appended periodically
- **Memory** — Small footprint, no large data structures

### Frontend (JavaScript)

- **Canvas rendering** — 30 ants max at 60fps, well within Canvas 2D capabilities
- **Animation loop** — Uses `requestAnimationFrame` for optimal performance
- **Event handling** — Passive listeners for scroll, debounced if needed
- **Memory** — Ant objects are lightweight, GC pressure minimal

### Optimization Opportunities

- **Object pooling** for ant entities (if count increases significantly)
- **Web Workers** for score calculation (if complexity grows)
- **GPU acceleration** via WebGL (if rendering becomes bottleneck)
- **Event throttling** for high-frequency events (scroll)

## 🔒 Security & Privacy

### Data Collection

- **Event types only** — scroll, keyboard, click, window focus
- **No keystroke content** — never captures what user types
- **No page content** — only window titles against known patterns
- **Local processing** — all computation on user's machine

### Tauri Security

- **Capability system** — frontend can only call exposed commands
- **No filesystem access** — except via explicit Tauri APIs
- **No network access** — no external requests in current implementation
- **Sandboxed WebView** — isolated from system

### Future Considerations

- If adding cloud sync, implement explicit opt-in
- If adding analytics, aggregate and anonymize
- Regular security audits for dependencies

## 📚 References

- [Tauri Documentation](https://tauri.app/v1/guides/)
- [Rust Book](https://doc.rust-lang.org/book/)
- [Canvas API](https://developer.mozilla.org/en-US/docs/Web/API/Canvas_API)
- [Web Audio API](https://developer.mozilla.org/en-US/docs/Web/API/Web_Audio_API)

---

For contribution guidelines, see [CONTRIBUTING.md](./CONTRIBUTING.md).
For user documentation, see [README.md](./README.md).
