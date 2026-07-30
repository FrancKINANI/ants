use tauri::Manager;

/// Phase 0 — Technical Spike: Transparent Overlay + Click-Through
///
/// Validates the feasibility of:
///   1. Transparent, always-on-top window
///   2. Per-region click-through (ignore cursor events on background,
///      allow clicks on specific UI elements — here, colored circles
///      that represent future ant sprites)
///
/// Platform support is evaluated automatically at build time.
/// See the OS support matrix in README.md for results.

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Get the main webview window
            let window = app.get_webview_window("main").unwrap();

            // Enable global click-through — mouse events pass through
            // the transparent background to the windows below.
            // This is the key requirement for the overlay (§8 in spec).
            window
                .set_ignore_cursor_events(true)
                .expect("Failed to set ignore cursor events");

            // Ensure always-on-top is enforced programmatically
            window
                .set_always_on_top(true)
                .expect("Failed to set always-on-top");

            // Log window info for diagnostics
            println!(
                "[Ants Phase 0] Overlay window created. Transparent: true | AlwaysOnTop: {} | ClickThrough: true",
                window.is_always_on_top().unwrap_or(false),
            );

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            toggle_click_through
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

/// Toggle click-through on the overlay window.
///
/// This will be used by the ant system to enable clicks on ant sprites
/// while keeping the background click-through. For Phase 0, this is a
/// simple toggle — in Phase 1, it will be called per-region based on
/// ant positions.
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
