/// System tray application (§11 of spec)
///
/// Tray icon with:
/// - Current attention score indicator
/// - Manual reset (fade all ants)
/// - Quick status overview
/// - Quit option

use tauri::{
    AppHandle, Emitter, Runtime,
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
    menu::{MenuBuilder, MenuItemBuilder},
    Manager,
};

/// Create and configure the system tray icon.
pub fn create_tray<R: Runtime>(app: &AppHandle<R>) -> Result<(), Box<dyn std::error::Error>> {
    // Build the tray menu
    let reset = MenuItemBuilder::with_id("reset", "Reset Ants")
        .accelerator("CmdOrCtrl+R")
        .build(app)?;

    let status = MenuItemBuilder::with_id("status", "Status: Active")
        .enabled(false)
        .build(app)?;

    let separator = tauri::menu::PredefinedMenuItem::separator(app)?;

    let quit = MenuItemBuilder::with_id("quit", "Quit Ants")
        .accelerator("CmdOrCtrl+Q")
        .build(app)?;

    let menu = MenuBuilder::new(app)
        .item(&status)
        .item(&separator)
        .item(&reset)
        .item(&separator)
        .item(&quit)
        .build()?;

    // Build the tray icon
    let _tray = TrayIconBuilder::new()
        .menu(&menu)
        .tooltip("Ants — Cognitive Interruptor")
        .icon(app.default_window_icon().unwrap().clone())
        .on_menu_event(move |app, event| {
            match event.id().as_ref() {
                "reset" => {
                    // Emit reset event to the main window
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.emit("ants:reset", ());
                    }
                }
                "quit" => {
                    app.exit(0);
                }
                _ => {}
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click {
                button: MouseButton::Left,
                button_state: MouseButtonState::Up,
                ..
            } = event
            {
                let app = tray.app_handle();
                // Toggle window visibility on click
                if let Some(window) = app.get_webview_window("main") {
                    if window.is_visible().unwrap_or(false) {
                        let _ = window.hide();
                    } else {
                        let _ = window.show();
                    }
                }
            }
        })
        .build(app)?;

    Ok(())
}
