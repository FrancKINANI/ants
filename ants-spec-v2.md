# Project Specification — Ants: A Cognitive Interruptor for Passive Screen Usage
## v2.1 — Revised Specification (with user decisions integrated)

### Changelog from v2

- **All open decisions (§18) are now resolved** with specific design choices from product owner.
- **Added §3: Emotional Design** — explicit framing of ants as *messengers* not *pests* (Option B).
- **Replaced §2 threshold model with Attention Score system** — continuous score instead of binary scroll-dominance ratio.
- **Updated §6 visual style** with concrete dimensions, color, frame count.
- **Updated §7 platform priority** to Windows → Linux X11 → macOS → Wayland (later).
- **Updated §3 instrumentation** to confirm local-only JSON log, no dashboard in v1.
- **Added §19: Attention Score — detailed spec** for the new detection system.

---

## 1. Vision

Build a lightweight desktop application that helps users regain awareness when they become trapped in passive, low-intent screen consumption (typically infinite scrolling).

Unlike website or app blockers, this application never restricts access to content. It introduces small, playful visual interruptions — inspired by the classic Windows "ants" desktop prank — that increase gradually as passive consumption continues, aiming to convert an automatic behavior back into a conscious choice.

**Core untested hypothesis:** a playful, low-friction interruption reduces automatic passive consumption more durably than doing nothing, and does so without triggering the reactance that blockers typically provoke. This spec ships a way to *check* that hypothesis (§4), not just assume it.

---

## 2. Design Philosophy (unchanged)

- The application should never feel like parental control software.
- It should feel alive; the user should smile.
- The interruption should be slightly annoying without becoming frustrating — friction, not punishment.
- The user must always remain in control.

---

## 3. Emotional Design — The Ants Are Messengers (resolved decision)

> **Decision:** Option B — The ants are *messengers*, not pests.

The ants are not "bad guys" to be eliminated. They are the visible symptom that the user is on autopilot. They never win, never lose. They appear, walk around, and disappear when the user regains conscious control.

This has concrete design consequences:
- **No score, no counter, no "ants killed" stat.** The instrumentation log (§4) records metrics for the hypothesis test, but these are never surfaced as a game score.
- **No punishment.** The ants do not multiply aggressively, do not become more "hostile," do not trigger alarm sounds.
- **No reward for squashing.** Clicking an ant is not a victory — it's just a way to dismiss one instance of the symptom. The user can also simply wait; ants fade on their own when attention returns.
- **The interaction should feel like brushing away a distraction**, not like fighting an enemy.

This choice is essential to distinguish the product from a mini-game. The product is a behavioral intervention, not entertainment.

---

## 4. Success Criteria & Instrumentation

Local-only. Never transmitted anywhere. Opt-out available.

**No visible dashboard in v1.** The purpose is to make the core hypothesis falsifiable internally, not to build a productivity tracker.

Minimal JSON log, per session:
- `date`: session timestamp
- `session_duration_minutes`: total session length
- `attention_score_timeline`: sampled readings of the attention score over time
- `ants_spawned`: total ants spawned
- `ants_dismissed` (by click or auto-fade)
- `time_until_user_left_seconds`: after first ant spawn, how long until the user switched context

Example log entry:
```json
{
  "date": "2026-08-01T14:30:00Z",
  "session_duration_minutes": 138,
  "attention_score_timeline": [
    {"t": 0, "score": 100},
    {"t": 30, "score": 72},
    {"t": 55, "score": 38},
    {"t": 62, "score": 21},
    {"t": 63, "score": 45}
  ],
  "ants_spawned": 37,
  "ants_dismissed": 31,
  "time_until_user_left_seconds": 46
}
```

Logs are written to a local file (`~/.ants/sessions.jsonl`). The user can export them manually if desired.

**Purpose:** make the core hypothesis falsifiable — if over weeks the user's behavior doesn't change (short time-to-dismiss, no context switches), the product isn't working and shouldn't be invested further.

---

## 5. Detection: Attention Score System

Replaces the §2 "scroll-dominance ratio" from v2 with a continuous Attention Score.

### Concept

A single score between 0 and 100 that represents how engaged the user is:
- **100** = fully engaged (typing, clicking with intent, active work)
- **0** = fully passive (continuous scrolling, no input)

The score is updated continuously based on a weighted combination of signals.

### Signals and Weights

| Signal | Weight | Source |
|---|---|---|
| Scroll events (continuous) | −25 / min above threshold | OS input events |
| Keyboard events absent | −5 / min of silence | OS input events |
| Window title contains "TikTok", "YouTube", "Instagram", "Facebook" | −10 / check | Window title heuristic (app name only) |
| Rapid window/tab switches (≥3 in 60s) | −15 / event | Window focus events |
| Session duration > 30 min without keyboard | −2 / min | Internal timer |
| Keyboard activity resumes | +15 / event burst | OS input events |
| Window switch to non-passive app (editor, terminal, IDE, Slack) | +10 / event | Window title heuristic |
| Click (deliberate, not scroll-linked) | +3 / click | OS input events |

**Privacy note:** Only event *types* and *rates* are tracked. Not keystroke content, not page content, not URLs — only:
- Event type (scroll, keyboard, click)
- Window title matched against a known list of passive-app patterns (e.g., "TikTok", "YouTube/Video")
- Window focus change events

All processing is local. No data leaves the machine.

### Score Dynamics

- **Decay:** signals push the score *down* continuously based on the rate/intensity of passive signals.
- **Recovery:** when active signals appear, the score recovers *faster* the lower it is (asymmetric recovery — easy to get back to green, harder to stay in the red).
- **Minimum:** 0 (no negative).
- **Maximum:** 100 (capped).

### Ant Spawning Thresholds

| Score Range | Behavior |
|---|---|
| 100–40 | No ants. User is engaged. |
| 39–25 | 1 ant spawns every 60 seconds. Subtle reminder. |
| 24–10 | 5 ants on screen. Moderate presence. |
| 9–0 | **Invasion.** Up to 30 ants, spawning every 10–15 seconds. Full intervention. |

When the score rises back above a threshold, ants disappear gradually (fade out over 3–5 seconds) rather than vanishing instantly.

**These thresholds are initial estimates and must be tuned through dogfooding.**

---

## 6. First MVP — Behavior

Implement only what's listed here plus §4 and §5. Avoid feature creep; focus on polish.

- The application runs silently in the background (system tray).
- The Attention Score (§5) runs continuously.
- When the score drops below 40, ants begin appearing per the thresholds.
- Each ant: walks naturally, changes direction occasionally, avoids leaving the screen, can overlap application windows, is always visible.
- Clicking an ant dismisses it (splat animation, optional sound).
- Remaining ants continue walking.
- When the score rises above a threshold, ants fade out gradually.
- A tray icon allows manual reset (immediate fade of all ants) and quick status check.

---

## 7. Visual Style (resolved decision)

**Decision: Stylized realism. Confirmed.**

- **Color:** matte black — no glossy or wet texture, no fine hair detail.
- **Size:** ~5–8 mm perceived size on a 24" 1080p screen (scale proportionally on higher DPI displays). On a typical laptop screen (13–15"), ~4–6 mm.
- **Texture:** simplified surface. No individual hairs, no compound eye rendering. A clean silhouette with subtle body segmentation (head, thorax, abdomen distinguishable).
- **Animation frames:** 6–10 frames per walk cycle. Smooth, not janky. Quality > quantity.
- **Anatomy:** six legs visible, correct proportion of head/thorax/abdomen, believable gait. Slightly enlarged head and antennae for readability at small sizes and for a touch of charm.
- **Movement:** organic. Random speed variation, random pauses, direction changes, slight body rotation. No perfectly straight lines.

---

## 8. Overlay Requirements — with platform scope (resolved decision)

Requirements:
- Click-through everywhere except ant hitboxes
- Always on top
- Invisible background
- GPU accelerated
- Minimal CPU usage
- Must never interfere with normal work except via the ants themselves

**Platform priority for v1:**
1. **Windows** (highest priority — `WS_EX_LAYERED` / `WS_EX_TRANSPARENT`)
2. **Linux X11** (second priority)
3. **macOS** (`NSWindow.ignoresMouseEvents`)
4. **Linux Wayland** — explicitly deferred. Not targeted for v1. Known risk: compositors restrict overlay positioning and per-pixel input transparency.

If the Phase 0 spike confirms feasibility on macOS, it will be included in v1. If not, it joins Wayland in the backlog.

---

## 9. Performance Requirements

- CPU usage: <1% idle, measured with 30 ants rendered simultaneously
- Memory: <100 MB
- No noticeable battery impact
- Target: 60 FPS animation

Baseline test environment: a mid-range 2020+ laptop with integrated GPU.

---

## 10. User Interaction

Clicking:

Mouse over ant → ant dies → small splat animation → sound (optional) → remove entity.

No blood. The splat is minimal — a small dark smudge that fades and disappears within ~0.5s. Consistent with §3 (messengers, not pests — the dismissal is gentle).

---

## 11. System Architecture

**Language:** Rust — performance, low memory footprint, cross-platform, native desktop development.

**UI shell:** Tauri v2 — Rust backend, modern desktop APIs, transparent windows, tray application support.

**Rendering:** wgpu (preferred), or Bevy ECS without pulling in the full game-engine feature set if unnecessary. Fallback: egui overlay if wgpu integration proves too costly for MVP timeline.

**Window management:** tao / winit.

**Animation architecture:** simple ECS. Each ant is an entity with components: Position, Velocity, Direction, State, AnimationFrame, Alive/Dead. Chosen for future extensibility (colonies, other insects).

**Platform risk owner:** whoever picks up Phase 0 (§18) owns validating that this stack actually delivers the click-through behavior in §8 on each target platform before the architecture is locked in.

---

## 12. Application Structure

```
src/
  core/              # App lifecycle, event bus
  overlay/           # Transparent window, click-through management
  score/             # Attention Score engine (§5)
  ants/              # Ant entity system, spawning, despawning
  animation/         # Walk cycles, transitions, splat effect
  input/             # OS event watchers (scroll, keyboard, focus)
  scheduler/         # Timing, score updates, spawning intervals
  settings/          # TOML config loader
  tray/              # System tray icon, menu, manual reset
  instrumentation/   # Local JSON logger (§4)
assets/
  sounds/            # Optional: gentle splat or ambience
  sprites/           # Ant frames (or procedural rendering)
```

---

## 13. Ant AI

Each ant has: Position, Direction, Energy, Random seed.

Behavior loop: Walk → Pause → Turn → Resume.

Pseudo-algorithm:
```
Walk 3–8 seconds
Random pause (0.5–2s)
Choose new angle (0–360°, biased away from recent direction)
Continue
Avoid screen borders (turn away when within 20px of edge)
Occasionally follow another ant (10% chance when near)
```

This produces emergent-looking behavior without needing real pathfinding or flocking logic.

---

## 14. Configuration

TOML-based settings.

```toml
# Attention score weights
scroll_penalty_per_min = 25.0
keyboard_silence_penalty_per_min = 5.0
passive_window_penalty = 10.0
rapid_switch_penalty = 15.0
session_decay_per_min = 2.0
keyboard_recovery = 15.0
active_window_recovery = 10.0
click_recovery = 3.0

# Spawning thresholds
spawn_threshold_moderate = 39
spawn_threshold_present = 24
spawn_threshold_invasion = 9
max_ants_moderate = 1
max_ants_present = 5
max_ants_invasion = 30

# General
enable_sound = false
enable_local_instrumentation = true
```

---

## 15. Passive Window Patterns

Known window title patterns that indicate passive consumption. Extendable by the user.

```toml
passive_window_patterns = [
  "tiktok", "youtube", "instagram", "facebook",
  "reddit", "twitter", "x.com", "netflix",
  "disney+", "hulu", "twitch", "pinterest",
  "snapchat", "whatsapp", "messenger", "discord"
]
```

---

## 16. Future Detection Signals — reclassified by sensitivity

**Low sensitivity — local event metadata only, no new consent flow needed:**
- Window title heuristics (app name only, not content) — already included in v1

**Medium sensitivity — requires explicit opt-in and a clear data-use disclosure before shipping:**
- Browser extension integration (knows which site, not page content, unless scoped further)

**High sensitivity — requires a dedicated privacy review and explicit consent UX before any development starts, not just a roadmap bullet:**
- Computer vision on screen content (reads what's actually on screen)
- Machine learning classifier on behavior patterns (infers detailed usage profiles)

None of these belong in MVP.

---

## 17. Nice Future Ideas (explicitly out of scope for MVP)

- Growing colony: ants emerge from screen edges instead of spawning randomly
- Colonies: multiple ant groups
- Queen ant: appears if ignored long enough
- Other insects: flies, cockroaches, spiders
- Seasonal events: winter ants, Halloween bugs
- Achievements (explicitly *not* included — see §3)
- Visible stats / focus mode, break reminders

---

## 18. Code Quality

- Clean, modular architecture
- Extensive comments (particularly for the score system — this is the core differentiator)
- Unit tests for Attention Score logic (§5) — this is the part most likely to be subtly wrong and hardest to debug once shipped
- Integration test: scripted "scroll + no keyboard" must trigger ants; "continuous typing" must not
- No unsafe Rust unless strictly necessary, and justified inline where used
- Cross-platform abstraction using the platform matrix from Phase 0

---

## 19. Deliverables

### Phase 0 — Technical spike (blocking, before any ant/animation code)

- Validate always-on-top + per-region click-through on:
  - Windows (primary target)
  - Linux X11 (secondary)
  - macOS (tertiary — if feasible)
- Deliverable: a short written OS support matrix stating what works, what doesn't, and the resulting v1 platform scope.

### Phase 1 — MVP

- Project architecture
- `Cargo.toml`
- Transparent overlay implementation, scoped to the platforms validated in Phase 0
- Attention Score engine (§5), with unit tests
- Ant entity system + animation engine
- Click detection / dismiss interaction
- Passive window title detection
- Local instrumentation log (§4)
- System tray application with manual reset
- Configuration system (§14)
- Build instructions
- README, including the OS support matrix from Phase 0 and known limitations

### Definition of Done for MVP

- Runs within the budget in §9, with 30 ants on screen, on every OS validated in Phase 0
- A scripted "type continuously for 30+ minutes" test does **not** trigger ant spawning
- A scripted "scroll continuously for 10+ minutes with no keyboard input" test **does** trigger spawning (score drops below thresholds)
- The instrumentation log is non-empty and queryable after a real usage session
- Manual reset via tray icon works and fades all ants within 5 seconds

---

## 20. Open Decisions — ALL RESOLVED

The following decisions from v2 are now closed:

1. ✅ **Visual direction** — Stylized realism confirmed. Matte black, 5–8mm on 24", 6–10 animation frames.
2. ✅ **Linux scope** — X11-only acceptable for v1. Windows #1, Linux X11 #2, macOS #3, Wayland deferred.
3. ✅ **Dashboard visibility** — No dashboard in v1. Local JSON logs only.
4. ✅ **Threshold model** — Attention Score system replaces binary scroll-dominance ratio.
5. ✅ **Emotional framing** — Option B: ants are messengers, not pests.
