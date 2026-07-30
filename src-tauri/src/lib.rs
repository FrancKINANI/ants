/// Ants — A Cognitive Interruptor for Passive Screen Usage
///
/// Phase 1 — MVP integration.
///
/// Modules:
/// - score:     Attention Score engine (0–100 continuous engagement score)
/// - settings:  TOML configuration system
/// - input:     OS-level input event watcher
/// - tray:      System tray icon with menu
/// - instrumentation: Local JSON session logging

mod score;
mod settings;
mod input;
mod tray;
mod instrumentation;

use score::ScoreEngine;
use settings::AntsConfig;
use instrumentation::Logger;
use std::sync::Mutex;
use tauri::Manager;

use std::time::Instant;

/// Application state shared between Tauri commands.
struct AppState {
    score_engine: Mutex<ScoreEngine>,
    logger: Mutex<Logger>,
    last_poll: Mutex<Instant>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Load configuration
    let config = AntsConfig::load();
    let logger = Logger::new(config.general.enable_local_instrumentation);
    let score_engine = ScoreEngine::new(config.score.clone());

    tauri::Builder::default()
        .manage(AppState {
            score_engine: Mutex::new(score_engine),
            logger: Mutex::new(logger),
            last_poll: Mutex::new(Instant::now()),
        })
        .setup(|app| {
            // Get the main webview window
            let window = app.get_webview_window("main").unwrap();

            // Note: set_ignore_cursor_events(true) is intentionally NOT called here.
            // On Wayland/GNOME it blocks ALL mouse events in the webview (not just
            // passes them through), which breaks scroll detection, ant clicking,
            // and even keyboard IPC. Instead, click-through is handled purely via
            // CSS `pointer-events: none` on the canvas background, with hit-testing
            // for ant regions (see ants.js _bindEvents).

            // Ensure always-on-top is enforced programmatically
            window
                .set_always_on_top(true)
                .expect("Failed to set always-on-top");

            // Create system tray
            tray::create_tray(app.handle()).unwrap_or_else(|e| {
                eprintln!("[Ants] Warning: could not create tray: {}", e);
            });

            // Log startup
            println!(
                "[Ants] Started. Score engine active. Instrumentation: {}",
                if app.state::<AppState>().logger.lock().unwrap().enabled {
                    "enabled"
                } else {
                    "disabled"
                }
            );

            // Resize window to cover the full screen (avoids fullscreen+transparent bug on Wayland)
            match window.primary_monitor() {
                Ok(Some(monitor)) => {
                    let size = monitor.size();
                    let _ = window.set_size(tauri::PhysicalSize {
                        width: size.width,
                        height: size.height,
                    });
                    let _ = window.set_position(tauri::PhysicalPosition { x: 0, y: 0 });
                    println!(
                        "[Ants] Resized overlay to {}x{}",
                        size.width, size.height
                    );
                }
                _ => {
                    eprintln!("[Ants] Warning: could not get primary monitor info");
                }
            }

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            feed_event,
            get_score_snapshot,
            reset_score,
            toggle_click_through,
            log_ant_spawn,
            log_ant_dismiss,
            log_user_left,
            force_low_score,
            flush_logger,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

// ── Tauri Commands ──

/// Feed an input event into the score engine from the frontend.
#[tauri::command]
fn feed_event(
    state: tauri::State<'_, AppState>,
    event_type: String,
    window_title: Option<String>,
) {
    let event = input::parse_frontend_event(&event_type, window_title.as_deref());
    if let Ok(mut engine) = state.score_engine.lock() {
        engine.feed(event);
    }
}

/// Get the current attention score snapshot.
#[tauri::command]
fn get_score_snapshot(state: tauri::State<'_, AppState>) -> score::ScoreSnapshot {
    let mut engine = state.score_engine.lock().unwrap();
    let mut last_poll = state.last_poll.lock().unwrap();

    // Use actual elapsed time since last poll for accurate score decay
    let now = Instant::now();
    let elapsed = now.duration_since(*last_poll);
    *last_poll = now;

    // Cap at 2 seconds to avoid huge jumps after pause/resume
    let dt = elapsed.min(std::time::Duration::from_secs(2));
    engine.update(dt);

    let snapshot = engine.snapshot();

    // Also log the score reading
    if let Ok(mut logger) = state.logger.lock() {
        logger.record_score(&snapshot);
    }

    snapshot
}

/// Reset the attention score to 100.
#[tauri::command]
fn reset_score(state: tauri::State<'_, AppState>) {
    if let Ok(mut engine) = state.score_engine.lock() {
        engine.reset();
    }
}

/// Toggle click-through on the overlay window.
#[tauri::command]
fn toggle_click_through(app: tauri::AppHandle, enabled: bool) -> Result<(), String> {
    let window = app
        .get_webview_window("main")
        .ok_or("Main window not found")?;
    window
        .set_ignore_cursor_events(enabled)
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Log that an ant was spawned.
#[tauri::command]
fn log_ant_spawn(state: tauri::State<'_, AppState>) {
    if let Ok(mut logger) = state.logger.lock() {
        logger.record_ant_spawned();
    }
}

/// Log that an ant was dismissed.
#[tauri::command]
fn log_ant_dismiss(state: tauri::State<'_, AppState>) {
    if let Ok(mut logger) = state.logger.lock() {
        logger.record_ant_dismissed();
    }
}

/// Log that the user left the passive context.
#[tauri::command]
fn log_user_left(state: tauri::State<'_, AppState>) {
    if let Ok(mut logger) = state.logger.lock() {
        logger.record_user_left();
    }
}

/// Debug: force the score to a very low value so ants appear immediately.
#[tauri::command]
fn force_low_score(state: tauri::State<'_, AppState>) {
    if let Ok(mut engine) = state.score_engine.lock() {
        engine.force_score(5.0);
    }
}

/// Flush the session log to disk.
/// Should be called when the app is closing.
#[tauri::command]
fn flush_logger(state: tauri::State<'_, AppState>) -> Result<(), String> {
    if let Ok(mut logger) = state.logger.lock() {
        logger.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}
