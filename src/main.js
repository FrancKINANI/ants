/**
 * Ants — Main Entry Point
 *
 * Integrates the Canvas ant engine with the Tauri v2 backend:
 * - Polls Attention Score from Rust backend every ~1s
 * - Feeds input events (keyboard, scroll, click) to the score engine
 * - Listens for tray "reset" events
 * - Handles keyboard shortcuts (Esc=quit, R=reset)
 */

import { AntEngine } from './ants.js';

// ── Tauri invoke helper ──

async function invoke(cmd, args = {}) {
  if (window.__TAURI__?.core) {
    try {
      return await window.__TAURI__.core.invoke(cmd, args);
    } catch (err) {
      console.warn(`[Ants] invoke('${cmd}') failed:`, err);
    }
  }
  return null;
}

// ── Main ──

document.addEventListener('DOMContentLoaded', () => {
  const canvas = document.getElementById('ant-canvas');
  if (!canvas) { console.error('[Ants] Canvas missing'); return; }

  const engine = new AntEngine(canvas);
  let pollTimer = 0;
  const POLL_INTERVAL = 1.0; // seconds

  // Attach score polling callback
  engine.onPollScore = async (eng) => {
    // The loop calls this every frame; throttle to once per second
    const dt = (performance.now() - eng.lastTime) / 1000 || 0.016;
    pollTimer += dt;
    if (pollTimer < POLL_INTERVAL) return;
    pollTimer = 0;

    const snap = await invoke('get_score_snapshot');
    if (!snap) return;

    eng.setLevel(snap.level, snap.score);

    // If user recovered significantly, log it
    if (snap.score > 60 && snap.level === 'none') {
      await invoke('log_user_left');
    }
  };

  // Listen for tray "reset" events
  if (window.__TAURI__?.core) {
    window.__TAURI__.core.listen('ants:reset', () => {
      engine.reset();
      invoke('reset_score');
    });
  }

  // Flush logs when the app closes (beforeunload + Tauri close)
  async function flushLogs() {
    await invoke('flush_logger');
  }
  window.addEventListener('beforeunload', flushLogs);

  // Start
  engine.start();

  // Keyboard shortcuts
  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      if (window.__TAURI__?.process) {
        window.__TAURI__.process.exit(0);
      }
    }
    if (e.key.toLowerCase() === 'r') {
      engine.reset();
      invoke('reset_score');
    }
  });

  console.log('[Ants] Engine started');
  console.log(`[Ants] Canvas: ${engine.width}x${engine.height} @ ${engine.dpr}x`);
});
