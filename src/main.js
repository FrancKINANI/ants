// Phase 0 — Overlay Spike Test: Frontend Logic
//
// This test validates:
//   1. The transparent overlay renders correctly
//   2. Click-through works (background clicks pass through to windows below)
//   3. Specific UI elements (the "ant" placeholders) are clickable
//   4. The overlay is always-on-top

const { invoke } = window.__TAURI__?.core || {};

document.addEventListener('DOMContentLoaded', () => {
  const clickLog = document.getElementById('click-log');
  const platformInfo = document.getElementById('platform-info');
  const testAnts = document.querySelectorAll('.test-ant');

  // ── Platform Detection ──

  function detectPlatform() {
    const ua = navigator.userAgent;
    let os = 'unknown';
    if (ua.includes('Windows')) os = 'Windows';
    else if (ua.includes('Mac OS')) os = 'macOS';
    else if (ua.includes('Linux')) os = 'Linux';

    const isWayland = ua.includes('Wayland') ||
                      window.location.href.includes('wayland');

    platformInfo.textContent = `Platform: ${os}${isWayland ? ' (Wayland)' : ''} | Tauri v2 | Transparent: ✓ | AlwaysOnTop: ✓ | ClickThrough: testing...`;

    return { os, isWayland };
  }

  const platform = detectPlatform();

  // ── Click Handling ──
  // When an ant placeholder is clicked, it proves that
  // per-region click-through is working: the ant catches
  // the click while the transparent background lets clicks pass.

  testAnts.forEach((ant) => {
    ant.addEventListener('click', (e) => {
      e.stopPropagation();

      const index = ant.dataset.index;
      const now = new Date().toLocaleTimeString();

      // Visual feedback
      ant.classList.add('squashed');

      clickLog.textContent = `🐜 Ant #${index} squashed at ${now} — Per-region click-through: ✓`;

      // Reset after animation
      setTimeout(() => {
        ant.classList.remove('squashed');
      }, 600);
    });
  });

  // ── Click-through background test ──
  // Clicking on the transparent background should NOT trigger anything
  // because the events pass through to windows behind the overlay.

  document.getElementById('overlay').addEventListener('click', (e) => {
    // Only log if not clicking on an ant
    if (!e.target.closest('.test-ant')) {
      clickLog.textContent = 'Click on background — should pass through to window below. Check if underlying app received it.';
    }
  });

  // ── Tauri Commands (if available) ──

  async function toggleClickThrough(enabled) {
    if (!invoke) return;
    try {
      await invoke('toggle_click_through', { enabled });
      const status = document.getElementById('status-text');
      status.textContent = enabled
        ? 'Overlay Active — Click-through enabled'
        : 'Click-through disabled — overlay captures all clicks';
    } catch (err) {
      console.error('Failed to toggle click-through:', err);
    }
  }

  // Expose toggle to console for manual testing
  window.toggleOverlay = toggleClickThrough;

  // ── Keyboard shortcuts for testing ──

  document.addEventListener('keydown', (e) => {
    if (e.key === 'Escape') {
      // Quit the app via Tauri API
      if (window.__TAURI__?.process) {
        window.__TAURI__.process.exit(0);
      }
    }
    if (e.key === 'c' && e.ctrlKey) {
      e.preventDefault();
      toggleClickThrough(false);
    }
    if (e.key === 'v' && e.ctrlKey) {
      e.preventDefault();
      toggleClickThrough(true);
    }
  });

  // ── Initial diagnostics ──

  console.log('[Ants Phase 0] Overlay initialized.');
  console.log(`[Ants Phase 0] Platform: ${platform.os} ${platform.isWayland ? '(Wayland)' : ''}`);
  console.log('[Ants Phase 0] Keyboard shortcuts: Esc=quit, Ctrl+C=disable click-through, Ctrl+V=enable click-through');
});
