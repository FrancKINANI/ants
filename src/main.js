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
    return;
  }
  console.warn(`[Ants] Tauri IPC not available — invoke('${cmd}') skipped`);
  return null;
}

// ── Main ──

document.addEventListener('DOMContentLoaded', () => {
  const canvas = document.getElementById('ant-canvas');
  if (!canvas) { console.error('[Ants] Canvas missing'); return; }

  const engine = new AntEngine(canvas);
  let pollTimer = 0;
  const POLL_INTERVAL = 1.0; // seconds

  // ── Demo mode (when Tauri backend is unavailable) ──
  let demoActive = false;
  let demoTime = 0;
  let failCount = 0;

  function triggerDemo() {
    demoActive = true;
    demoTime = 75; // Score starts at ~25 (moderate level — 1 ant appears)
    engine.setLevel('moderate', 25);
    console.log('[Ants] Demo mode activated');
  }

  engine.onPollScore = async (eng) => {
    const dt = (performance.now() - eng.lastTime) / 1000 || 0.016;
    pollTimer += dt;
    if (pollTimer < POLL_INTERVAL) return;
    pollTimer = 0;

    if (!demoActive) {
      const snap = await invoke('get_score_snapshot');
      if (snap) {
        eng.setLevel(snap.level, snap.score);
        if (snap.score > 60 && snap.level === 'none') {
          await invoke('log_user_left');
        }
        return;
      }
      // Backend unavailable — activate demo after 5 failed polls
      if (++failCount >= 5) {
        triggerDemo();
      }
      return;
    }

    // Demo mode: simulate score decay at 2 pts/sec
    demoTime += POLL_INTERVAL * 2;
    const simulatedScore = Math.max(0, 100 - demoTime);
    const level = simulatedScore >= 40 ? 'none'
      : simulatedScore >= 25 ? 'moderate'
      : simulatedScore >= 10 ? 'present'
      : 'invasion';
    eng.setLevel(level, simulatedScore);
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
    // Debug: press D to force ants to appear immediately
    if (e.key.toLowerCase() === 'd' && !e.ctrlKey && !e.metaKey) {
      if (window.__TAURI__?.core) {
        invoke('force_low_score');
        console.log('[Ants] Debug: force_low_score via backend');
      } else {
        // Fallback: trigger demo mode directly
        triggerDemo();
        // Override to invasion level for immediate effect
        engine.setLevel('invasion', 5);
        console.log('[Ants] Debug: demo mode triggered directly');
      }
    }
  });

  console.log('[Ants] Engine started');
  console.log(`[Ants] Canvas: ${engine.width}x${engine.height} @ ${engine.dpr}x`);
});
