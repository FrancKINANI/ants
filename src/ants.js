/**
 * Ants — Canvas Rendering Engine
 *
 * Procedural ant drawing with organic movement AI, spawn/despawn system,
 * and click interaction. Renders on a full-screen transparent Canvas overlay.
 *
 * Style: stylized realism — matte black, 6 legs, organic walk cycle.
 * Philosophy: ants are messengers, not pests (Option B). No blood, no gore.
 */

// ── Configuration ──

const CFG = {
  baseLength: 28,
  bodyColor: '#1a1a1a',
  legColor: '#2a2a2a',
  walkSpeed: 40,
  speedVariance: 0.4,
  walkDurationMin: 3,
  walkDurationMax: 8,
  pauseDurationMin: 0.5,
  pauseDurationMax: 2.0,
  turnAmount: Math.PI / 3,
  borderMargin: 60,
  spawnIntervalModerate: 3000,
  spawnIntervalPresent: 1500,
  spawnIntervalInvasion: 500,
  maxAntsModerate: 1,
  maxAntsPresent: 5,
  maxAntsInvasion: 30,
  fadeInDuration: 500,
  fadeOutDuration: 800,
  squashDuration: 400,
};

let nextId = 0;

function rand(min, max) { return min + Math.random() * (max - min); }

function createAnt(x, y, dpr) {
  return {
    id: nextId++,
    x, y,
    angle: Math.random() * Math.PI * 2,
    speed: CFG.walkSpeed * dpr * (1 + rand(-0.5, 0.5) * CFG.speedVariance * 2),
    targetAngle: 0,
    state: 'fadingIn',
    stateTimer: rand(CFG.walkDurationMin, CFG.walkDurationMax),
    opacity: 0,
    scale: 1,
    walkFrame: Math.random() * Math.PI * 2,
    legPhase: 0,
    turnTimer: 0,
  };
}

// ── Procedural Drawing ──

function drawAnt(ctx, ant, dpr) {
  const s = CFG.baseLength * dpr * ant.scale / 28;
  const { x, y, angle, walkFrame, opacity } = ant;

  ctx.save();
  ctx.translate(x, y);
  ctx.rotate(angle);
  ctx.globalAlpha = Math.max(0, Math.min(1, opacity));

  const headR = 4.5 * s, thoraxR = 5 * s, abdomenR = 7 * s;
  const legLen = 14 * s, legOff = 6 * s;

  // Legs
  const leg = (ph) => Math.sin(ph) * 0.4 + 0.6;
  const l1 = leg(walkFrame), l2 = leg(walkFrame + 2.1), l3 = leg(walkFrame + 4.2);

  ctx.strokeStyle = CFG.legColor;
  ctx.lineWidth = 1.8 * s;
  ctx.lineCap = 'round';

  for (const [off, side] of [[-legOff, -1], [legOff, 1]]) {
    for (const [idx, swing] of [[0, l1], [1, l2], [2, l3]]) {
      const yb = idx === 1 ? 0 : (idx === 0 ? -2 : 2) * side * 0.3;
      ctx.beginPath();
      ctx.moveTo(off, yb);
      ctx.quadraticCurveTo(
        off + legLen * 0.5 * swing * side,
        yb + legLen * 0.4 * side,
        off + legLen * swing * side,
        yb + legLen * (1 - swing * 0.3) * side * 0.5
      );
      ctx.stroke();
    }
  }

  // Body
  ctx.fillStyle = CFG.bodyColor;
  ctx.beginPath();
  ctx.ellipse(thoraxR + 2 * s, 0, abdomenR, abdomenR * 0.8, 0, 0, Math.PI * 2);
  ctx.fill();
  ctx.beginPath();
  ctx.ellipse(2 * s, 0, thoraxR, thoraxR * 0.75, 0, 0, Math.PI * 2);
  ctx.fill();
  const hOff = -(thoraxR + headR - 2 * s);
  ctx.beginPath();
  ctx.ellipse(hOff, 0, headR, headR * 0.85, 0, 0, Math.PI * 2);
  ctx.fill();

  // Eyes
  ctx.fillStyle = '#000';
  ctx.beginPath();
  ctx.arc(hOff - 1 * s, -2 * s, 1.2 * s, 0, Math.PI * 2);
  ctx.arc(hOff - 1 * s, 2 * s, 1.2 * s, 0, Math.PI * 2);
  ctx.fill();

  // Antennae
  ctx.strokeStyle = CFG.legColor;
  ctx.lineWidth = 1.2 * s;
  const sway = Math.sin(walkFrame * 0.7) * 2 * s;
  const bx = hOff - headR + 1 * s;
  for (const dir of [-1, 1]) {
    ctx.beginPath();
    ctx.moveTo(bx, dir * 1 * s);
    ctx.quadraticCurveTo(
      bx - 9 * s, dir * 6 * s + sway,
      bx - 12 * s, dir * 3 * s + sway * 0.5
    );
    ctx.stroke();
  }

  ctx.restore();
}

// ── AI Update ──

function updateAnt(ant, dt, w, h, dpr) {
  switch (ant.state) {
    case 'fadingIn':
      ant.opacity += dt / (CFG.fadeInDuration / 1000);
      if (ant.opacity >= 1) { ant.opacity = 1; ant.state = 'walking'; }
      return;
    case 'fadingOut':
      ant.opacity -= dt / (CFG.fadeOutDuration / 1000);
      if (ant.opacity <= 0) { ant.opacity = 0; ant.state = 'dead'; }
      return;
    case 'squashed':
      ant.scale -= dt / (CFG.squashDuration / 1000);
      ant.opacity -= dt / (CFG.squashDuration / 1000);
      if (ant.scale <= 0 || ant.opacity <= 0) ant.state = 'dead';
      return;
    case 'dead':
      return;
  }

  ant.stateTimer -= dt;
  if (ant.stateTimer <= 0) {
    if (ant.state === 'walking') {
      ant.state = 'pausing';
      ant.stateTimer = rand(CFG.pauseDurationMin, CFG.pauseDurationMax);
    } else {
      ant.state = 'walking';
      ant.stateTimer = rand(CFG.walkDurationMin, CFG.walkDurationMax);
      ant.targetAngle = ant.angle + rand(-1, 1) * CFG.turnAmount;
    }
  }

  if (ant.state === 'walking') {
    let diff = ant.targetAngle - ant.angle;
    while (diff > Math.PI) diff -= Math.PI * 2;
    while (diff < -Math.PI) diff += Math.PI * 2;
    ant.angle += diff * 3 * dt;
    ant.walkFrame += ant.speed * 0.06 * dt;
    ant.legPhase += dt * 8;
    ant.x += Math.cos(ant.angle) * ant.speed * dt;
    ant.y += Math.sin(ant.angle) * ant.speed * dt;

    const m = CFG.borderMargin * dpr, f = 3;
    if (ant.x < m) ant.angle += f * dt;
    if (ant.x > w - m) ant.angle -= f * dt;
    if (ant.y < m) ant.angle += f * dt;
    if (ant.y > h - m) ant.angle -= f * dt;
  }
}

// ── Hitbox & Squash ──

function hitTest(ant, mx, my, dpr) {
  if (ant.state === 'dead' || ant.opacity < 0.2) return false;
  const r = 16 * dpr * ant.scale;
  const dx = mx - ant.x, dy = my - ant.y;
  return dx * dx + dy * dy <= r * r;
}

// ── Engine ──

export class AntEngine {
  constructor(canvas) {
    this.canvas = canvas;
    this.ctx = canvas.getContext('2d');
    this.ants = [];
    this.dpr = window.devicePixelRatio || 1;
    this.lastTime = performance.now();
    this.spawnTimer = 0;
    this.currentLevel = 'none';
    this.running = false;
    this.onPollScore = null;

    this._els = {
      log: document.getElementById('click-log'),
      score: document.getElementById('score-value'),
      status: document.getElementById('status-text'),
      count: document.getElementById('ants-count'),
      dot: document.getElementById('status-indicator'),
    };

    this._resize();
    this._bindEvents();
  }

  _resize() {
    const w = window.innerWidth, h = window.innerHeight;
    this.canvas.width = w * this.dpr;
    this.canvas.height = h * this.dpr;
    this.canvas.style.width = w + 'px';
    this.canvas.style.height = h + 'px';
    this.width = w * this.dpr;
    this.height = h * this.dpr;
  }

  _invoke(cmd, args) {
    if (window.__TAURI__?.core) {
      return window.__TAURI__.core.invoke(cmd, args).catch(() => {});
    }
  }

  _log(msg) { if (this._els.log) this._els.log.textContent = msg; }

  setLevel(level, score) {
    this.currentLevel = level;
    if (this._els.score) this._els.score.textContent = Math.round(score);
    if (this._els.status) {
      this._els.status.textContent = {
        none: 'Focused', moderate: 'Watching...',
        present: 'Distracted', invasion: 'Invasion!',
      }[level] || 'Active';
    }
    if (this._els.dot) {
      this._els.dot.className = 'status-dot ' + ({
        none: 'active', moderate: 'watching',
        present: 'distracted', invasion: 'invasion',
      }[level] || 'active');
    }
  }

  start() {
    if (this.running) return;
    this.running = true;
    this.lastTime = performance.now();
    this._loop();
  }

  stop() { this.running = false; }

  reset() {
    for (const a of this.ants) {
      if (a.state === 'walking' || a.state === 'pausing' || a.state === 'fadingIn') {
        a.state = 'fadingOut';
      }
    }
  }

  // ── Internals ──

  _bindEvents() {
    window.addEventListener('resize', () => this._resize());

    // ── Click-through management ──
    // By default, canvas has pointer-events:none so clicks pass through
    // to windows behind the overlay. When the cursor nears an ant,
    // we enable pointer-events so the ant can catch the click.
    // This gives us per-region-like click-through without platform hacks.

    document.addEventListener('mousemove', (e) => {
      const mx = e.clientX * this.dpr, my = e.clientY * this.dpr;
      let nearAnt = false;
      for (const a of this.ants) {
        if (a.state !== 'dead' && a.opacity >= 0.2) {
          const dx = mx - a.x, dy = my - a.y;
          const hitR = 24 * this.dpr * a.scale; // larger than hitbox for comfortable targeting
          if (dx * dx + dy * dy <= hitR * hitR) {
            nearAnt = true;
            break;
          }
        }
      }
      this.canvas.style.pointerEvents = nearAnt ? 'auto' : 'none';
      this.canvas.style.cursor = nearAnt ? 'pointer' : 'default';
    });

    this.canvas.addEventListener('click', (e) => {
      const mx = e.clientX * this.dpr, my = e.clientY * this.dpr;
      for (const a of this.ants) {
        if (hitTest(a, mx, my, this.dpr)) {
          a.state = 'squashed';
          this._log(`Dismissed ant #${a.id}`);
          this._invoke('feed_event', { eventType: 'click' });
          this._invoke('log_ant_dismiss');
          e.stopPropagation();
          return;
        }
      }
    });

    document.addEventListener('keydown', () => this._invoke('feed_event', { eventType: 'keyboard' }));
    window.addEventListener('wheel', () => this._invoke('feed_event', { eventType: 'scroll' }));
  }

  _maxAnts() {
    return { moderate: CFG.maxAntsModerate, present: CFG.maxAntsPresent, invasion: CFG.maxAntsInvasion }[this.currentLevel] || 0;
  }

  _spawnInterval() {
    return { moderate: CFG.spawnIntervalModerate, present: CFG.spawnIntervalPresent, invasion: CFG.spawnIntervalInvasion }[this.currentLevel] || Infinity;
  }

  _spawn(dt) {
    const max = this._maxAnts();
    const interval = this._spawnInterval();
    this.ants = this.ants.filter(a => a.state !== 'dead');
    if (this.ants.length >= max) return;

    this.spawnTimer += dt * 1000;
    if (this.spawnTimer < interval) return;
    this.spawnTimer = 0;

    const m = 20 * this.dpr;
    const edge = Math.floor(Math.random() * 4);
    const pos = [
      [Math.random() * this.width, -m],
      [this.width + m, Math.random() * this.height],
      [Math.random() * this.width, this.height + m],
      [-m, Math.random() * this.height],
    ][edge];

    this.ants.push(createAnt(pos[0], pos[1], this.dpr));
    this._invoke('log_ant_spawn');
  }

  _loop() {
    if (!this.running) return;
    const now = performance.now();
    const dt = Math.min((now - this.lastTime) / 1000, 0.05);
    this.lastTime = now;

    // Poll score from backend
    if (this.onPollScore) this.onPollScore(this);

    // Clear, spawn, update, draw
    this.ctx.clearRect(0, 0, this.width, this.height);
    this._spawn(dt);
    for (const a of this.ants) {
      updateAnt(a, dt, this.width, this.height, this.dpr);
      if (a.state !== 'dead') drawAnt(this.ctx, a, this.dpr);
    }

    // Count display
    if (this._els.count) {
      const alive = this.ants.filter(a => a.state !== 'dead').length;
      this._els.count.textContent = `${alive} ant${alive !== 1 ? 's' : ''} on screen`;
    }

    requestAnimationFrame(() => this._loop());
  }
}
