/// Local Instrumentation (§4 of spec)
///
/// Minimal, privacy-preserving session logging. Never transmitted anywhere.
/// Logs are written as JSON Lines to `~/.ants/sessions.jsonl`.
///
/// Purpose: make the core hypothesis falsifiable — if over weeks the user's
/// behavior doesn't change (short time-to-dismiss, no context switches),
/// the product isn't working and shouldn't be invested further.

use crate::score::ScoreSnapshot;
use serde::Serialize;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::PathBuf;
use std::time::{SystemTime, UNIX_EPOCH};

/// A single session log entry.
#[derive(Debug, Clone, Serialize)]
pub struct SessionLog {
    /// ISO 8601 timestamp of session start
    pub date: String,
    /// Total session duration in minutes
    pub session_duration_minutes: f64,
    /// Timestamped attention score readings during the session
    pub attention_score_timeline: Vec<ScoreReading>,
    /// Total ants spawned during session
    pub ants_spawned: u32,
    /// Ants dismissed (by click or auto-fade)
    pub ants_dismissed: u32,
    /// Time from first ant spawn to user leaving the context (seconds)
    pub time_until_user_left_seconds: Option<f64>,
}

/// A single attention score reading at a point in time.
#[derive(Debug, Clone, Serialize)]
pub struct ScoreReading {
    /// Seconds since session start
    pub t: f64,
    /// Score value at that time
    pub score: f64,
}

/// The instrumentation logger.
#[derive(Debug)]
pub struct Logger {
    log_path: PathBuf,
    session_start: SystemTime,
    timeline: Vec<ScoreReading>,
    ants_spawned: u32,
    ants_dismissed: u32,
    first_ant_time: Option<SystemTime>,
    user_left_time: Option<SystemTime>,
    pub enabled: bool,
}

impl Logger {
    /// Create a new logger.
    pub fn new(enabled: bool) -> Self {
        let home = std::env::var("HOME").unwrap_or_else(|_| ".".to_string());
        let mut path = PathBuf::from(home);
        path.push(".ants");
        path.push("sessions.jsonl");

        Self {
            log_path: path,
            session_start: SystemTime::now(),
            timeline: Vec::new(),
            ants_spawned: 0,
            ants_dismissed: 0,
            first_ant_time: None,
            user_left_time: None,
            enabled,
        }
    }

    /// Record an attention score reading.
    pub fn record_score(&mut self, snapshot: &ScoreSnapshot) {
        if !self.enabled {
            return;
        }
        self.timeline.push(ScoreReading {
            t: snapshot.elapsed_secs,
            score: snapshot.score,
        });
    }

    /// Record that an ant was spawned.
    pub fn record_ant_spawned(&mut self) {
        if !self.enabled {
            return;
        }
        self.ants_spawned += 1;
        if self.first_ant_time.is_none() {
            self.first_ant_time = Some(SystemTime::now());
        }
    }

    /// Record that an ant was dismissed.
    pub fn record_ant_dismissed(&mut self) {
        if !self.enabled {
            return;
        }
        self.ants_dismissed += 1;
    }

    /// Record that the user left the passive context.
    pub fn record_user_left(&mut self) {
        if !self.enabled {
            return;
        }
        self.user_left_time = Some(SystemTime::now());
    }

    /// Finalize and write the session log to disk.
    pub fn flush(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if !self.enabled {
            return Ok(());
        }

        let now = SystemTime::now();
        let session_duration = now
            .duration_since(self.session_start)
            .unwrap_or_default()
            .as_secs_f64()
            / 60.0;

        let time_until_left = self.first_ant_time.and_then(|first| {
            let leave = self.user_left_time.unwrap_or(now);
            leave
                .duration_since(first)
                .ok()
                .map(|d| d.as_secs_f64())
        });

        // Format timestamp
        let timestamp = self
            .session_start
            .duration_since(UNIX_EPOCH)
            .unwrap_or_default()
            .as_secs();

        let log = SessionLog {
            date: format!("{}", timestamp),
            session_duration_minutes: session_duration,
            attention_score_timeline: std::mem::take(&mut self.timeline),
            ants_spawned: self.ants_spawned,
            ants_dismissed: self.ants_dismissed,
            time_until_user_left_seconds: time_until_left,
        };

        // Ensure directory exists
        if let Some(parent) = self.log_path.parent() {
            fs::create_dir_all(parent)?;
        }

        // Append to the log file
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.log_path)?;

        let json = serde_json::to_string(&log)?;
        writeln!(file, "{}", json)?;

        // Reset for next session
        self.session_start = SystemTime::now();
        self.ants_spawned = 0;
        self.ants_dismissed = 0;
        self.first_ant_time = None;
        self.user_left_time = None;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_logger_creation() {
        let logger = Logger::new(true);
        assert!(logger.enabled);
        assert_eq!(logger.ants_spawned, 0);
    }

    #[test]
    fn test_disabled_logger() {
        let mut logger = Logger::new(false);
        logger.record_ant_spawned();
        assert_eq!(logger.ants_spawned, 0);
    }

    #[test]
    fn test_logger_counts() {
        let mut logger = Logger::new(true);
        logger.record_ant_spawned();
        logger.record_ant_spawned();
        logger.record_ant_spawned();
        logger.record_ant_dismissed();
        assert_eq!(logger.ants_spawned, 3);
        assert_eq!(logger.ants_dismissed, 1);
    }
}
