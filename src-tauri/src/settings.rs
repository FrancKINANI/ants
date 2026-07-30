/// Configuration system (§14 of spec)
///
/// TOML-based settings file stored at `~/.ants/config.toml`.
/// Falls back to sensible defaults if the file doesn't exist.

use crate::score::ScoreConfig;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Root configuration for the Ants application.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntsConfig {
    /// Attention Score engine settings
    pub score: ScoreConfig,

    /// General settings
    pub general: GeneralConfig,

    /// Window title patterns that indicate passive consumption
    pub passive_window_patterns: Vec<String>,

    /// Window title patterns that indicate active work
    pub active_window_patterns: Vec<String>,
}

/// General application settings.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GeneralConfig {
    /// Enable sound effects (requires sound files)
    pub enable_sound: bool,
    /// Enable local instrumentation logging
    pub enable_local_instrumentation: bool,
    /// Spawn interval between individual ants (seconds)
    pub spawn_interval_seconds: f64,
    /// Maximum ants on screen at once
    pub max_ants: u32,
    /// Overlay opacity (0.0 - 1.0)
    pub overlay_opacity: f64,
}

impl Default for GeneralConfig {
    fn default() -> Self {
        Self {
            enable_sound: false,
            enable_local_instrumentation: true,
            spawn_interval_seconds: 5.0,
            max_ants: 30,
            overlay_opacity: 1.0,
        }
    }
}

impl Default for AntsConfig {
    fn default() -> Self {
        Self {
            score: ScoreConfig::default(),
            general: GeneralConfig::default(),
            passive_window_patterns: vec![
                "tiktok".into(), "youtube".into(), "instagram".into(),
                "facebook".into(), "reddit".into(), "twitter".into(),
                "x.com".into(), "netflix".into(), "disney+".into(),
                "hulu".into(), "twitch".into(), "pinterest".into(),
                "snapchat".into(), "whatsapp".into(), "messenger".into(),
                "discord".into(),
            ],
            active_window_patterns: vec![
                "code".into(), "vim".into(), "nvim".into(), "emacs".into(),
                "terminal".into(), "bash".into(), "zsh".into(),
                "idea".into(), "intellij".into(), "slack".into(),
                "notion".into(), "obsidian".into(), "word".into(),
                "excel".into(), "outlook".into(),
            ],
        }
    }
}

impl AntsConfig {
    /// Get the path to the config file.
    pub fn config_path() -> PathBuf {
        let home = std::env::var("HOME")
            .unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push(".ants");
        path.push("config.toml");
        path
    }

    /// Load config from the default path, or create default if not found.
    pub fn load() -> Self {
        let path = Self::config_path();

        if !path.exists() {
            let config = AntsConfig::default();
            if let Err(e) = config.save() {
                eprintln!("[Ants] Warning: could not create default config: {}", e);
            }
            return config;
        }

        match std::fs::read_to_string(&path) {
            Ok(content) => {
                match toml::from_str(&content) {
                    Ok(config) => config,
                    Err(e) => {
                        eprintln!(
                            "[Ants] Warning: config parse error ({}), using defaults",
                            e
                        );
                        AntsConfig::default()
                    }
                }
            }
            Err(e) => {
                eprintln!("[Ants] Warning: could not read config ({}), using defaults", e);
                AntsConfig::default()
            }
        }
    }

    /// Save the current config to the default path.
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let path = Self::config_path();

        // Ensure directory exists
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = toml::to_string_pretty(self)?;
        std::fs::write(&path, content)?;

        Ok(())
    }
}
