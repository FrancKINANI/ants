/// OS-level Input Event Parser (§2, §5 of spec)
///
/// Parses frontend-reported input events and converts them into
/// `InputEvent` values for the Attention Score engine.
///
/// Only tracks event *types and rates* — no content, no keystroke logging.
///
/// In Phase 1, events are reported by the frontend (JS) via Tauri commands.
/// Future versions may use direct OS hooks (evdev, IOHID) for lower latency.

use crate::score::InputEvent;

/// Parse a frontend-reported event type into an InputEvent.
///
/// Called from the `feed_event` Tauri command handler.
pub fn parse_frontend_event(event_type: &str, window_title: Option<&str>) -> InputEvent {
    match event_type {
        "scroll" => InputEvent::Scroll,
        "keyboard" => InputEvent::Keyboard,
        "click" => InputEvent::Click,
        "focus" => InputEvent::WindowFocus(
            window_title.unwrap_or("unknown").to_string(),
        ),
        _ => {
            eprintln!("[Ants] Warning: unknown event type '{}'", event_type);
            InputEvent::Keyboard // safe default
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_scroll() {
        assert_eq!(parse_frontend_event("scroll", None), InputEvent::Scroll);
    }

    #[test]
    fn test_parse_keyboard() {
        assert_eq!(parse_frontend_event("keyboard", None), InputEvent::Keyboard);
    }

    #[test]
    fn test_parse_click() {
        assert_eq!(parse_frontend_event("click", None), InputEvent::Click);
    }

    #[test]
    fn test_parse_focus() {
        match parse_frontend_event("focus", Some("Code".into())) {
            InputEvent::WindowFocus(title) => assert_eq!(title, "Code"),
            _ => panic!("Expected WindowFocus"),
        }
    }

    #[test]
    fn test_parse_unknown_defaults_to_keyboard() {
        assert_eq!(parse_frontend_event("unknown", None), InputEvent::Keyboard);
    }
}
