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

/// Application state shared between Tauri commands.
struct AppState {
    score_engine: Mutex<ScoreEngine>,
    logger: Mutex<Logger>,
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
        })
        .setup(|app| {
            // Get the main webview window
            let window = app.get_webview_window("main").unwrap();

            // Enable global click-through — mouse events pass through
            // the transparent background to the windows below.
            window
                .set_ignore_cursor_events(true)
                .expect("Failed to set ignore cursor events");

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
    // Update with a small delta to process time-based decay
    engine.update(std::time::Duration::from_millis(16)); // ~60fps tick
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

/// Flush the session log to disk.
/// Should be called when the app is closing.
#[tauri::command]
fn flush_logger(state: tauri::State<'_, AppState>) -> Result<(), String> {
    if let Ok(mut logger) = state.logger.lock() {
        logger.flush().map_err(|e| e.to_string())?;
    }
    Ok(())
}
