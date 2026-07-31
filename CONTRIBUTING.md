# Contributing to Ants

Thank you for your interest in contributing to Ants! This document provides guidelines and information to help you get started.

## 🎯 Project Philosophy

Ants is a behavioral intervention tool, not a game or blocker. When contributing, keep these principles in mind:

- **Gentle friction, not punishment** — The app should feel like a helpful nudge, not parental control
- **User always in control** — Never restrict access or force behavior
- **Privacy-first** — No content inspection, no keystroke logging, no data transmission
- **Playful, not hostile** — Ants are messengers, not enemies to be defeated

## 🚀 Getting Started

### Prerequisites

- **Rust** (latest stable): Install from [rustup.rs](https://rustup.rs/)
- **Node.js** (optional, for frontend tooling): Not required for basic development
- **Platform-specific dependencies:** See [README.md](./README.md#installation--usage)

### Development Setup

```bash
# Clone the repository
git clone https://github.com/yourusername/ants.git
cd ants

# Install Rust dependencies (handled automatically by Cargo)
cargo build

# Run in development mode
cargo run
```

### Development Workflow

```bash
# Make your changes
# ...

# Run tests
cargo test

# Run linter (if clippy is installed)
cargo clippy

# Build release version to test performance
cargo build --release
cargo run --release
```

## 📁 Codebase Overview

The project is split into two main parts:

### Backend (Rust) — `src-tauri/src/`

- **`main.rs`** — Application entry point
- **`lib.rs`** — Tauri setup, command handlers, shared state
- **`score.rs`** — Attention Score engine
- **`input.rs`** — Input event parsing
- **`settings.rs`** — Configuration management
- **`tray.rs`** — System tray implementation
- **`instrumentation.rs`** — Session logging

### Frontend (JavaScript) — `src/`

- **`main.js`** — Frontend orchestration, Tauri communication
- **`ants.js`** — Canvas rendering engine, ant simulation
- **`index.html`** — Main overlay interface
- **`styles.css`** — UI styling

For detailed architecture information, see [ARCHITECTURE.md](./ARCHITECTURE.md).

## 🧪 Testing

### Running Tests

```bash
# Run all Rust tests
cargo test

# Run tests with output
cargo test -- --nocapture

# Run specific test
cargo test test_parse_scroll
```

### Adding Tests

When adding new functionality, please include tests:

```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_your_function() {
        assert_eq!(your_function(), expected_value);
    }
}
```

### Manual Testing

Use the **`D` key** to trigger demo mode (spawn ants regardless of score) for testing the rendering system without waiting for the attention score to drop.

## 🐛 Bug Reports

When reporting bugs, please include:

1. **Platform and OS version** (e.g., Ubuntu 24.04, Windows 11, macOS Sonoma)
2. **Steps to reproduce** the issue
3. **Expected behavior** vs **actual behavior**
4. **Relevant logs** from `~/.ants/sessions.jsonl` (if applicable)
5. **Configuration** from `~/.ants/config.toml` (if relevant)

## 💡 Feature Requests

Before proposing new features, consider:

- Does it align with the **gentle friction** philosophy?
- Does it respect **user privacy**?
- Does it maintain the **messenger, not pest** framing?

For major features, open an issue first to discuss before implementing.

## 📝 Coding Style

### Rust

- Follow standard Rust conventions (`rustfmt` recommended)
- Use `cargo clippy` to catch common issues
- Prefer idiomatic Rust patterns
- Document public APIs with comments

### JavaScript

- Use modern ES6+ syntax
- Follow existing code style in the project
- Add comments for complex logic
- Keep functions focused and small

## 🔧 Configuration Changes

When modifying the configuration system:

1. Update `src-tauri/src/settings.rs` to add new fields
2. Update `ScoreConfig::default()` with sensible defaults
3. Document the new setting in this file or README
4. Consider backward compatibility for existing config files

## 🎨 Visual Changes

When modifying ant rendering:

- Maintain the **stylized realism** aesthetic (matte black, clean silhouette)
- Keep animations smooth (6-10 frames per walk cycle)
- Test on different screen sizes/DPI settings
- Ensure organic movement (randomness, pauses, direction changes)

## 🔒 Privacy Considerations

When working with input handling or window detection:

- **Never log keystroke content** — Only event types and rates
- **Never inspect page content** — Only window titles against known patterns
- **Never transmit data** — All processing must be local
- **Be transparent** — Document what data is collected and why

## 📖 Documentation

When adding new features:

1. Update relevant code comments
2. Update [ARCHITECTURE.md](./ARCHITECTURE.md) if the architecture changes
3. Update [README.md](./README.md) if user-facing behavior changes
4. Update this file if contribution guidelines change

## 🔄 Pull Request Process

1. **Fork** the repository
2. **Create a branch** for your feature (`git checkout -b feature/amazing-feature`)
3. **Make your changes** following the guidelines above
4. **Test** thoroughly (`cargo test`, manual testing)
5. **Commit** with clear, descriptive messages
6. **Push** to your fork
7. **Open a Pull Request** with:
   - Description of changes
   - Related issues (if any)
   - Testing performed
   - Screenshots (if visual changes)

### Commit Message Style

```
Brief description (50 chars or less)

More detailed explanatory text, if necessary. Wrap it to about 72
characters or so.

- Bullet points for multiple changes
- Reference issues with #123
```

## 🏗️ Architecture Decisions

When making architectural changes:

- Consider the **separation of concerns** between Rust (logic) and JavaScript (rendering)
- Maintain the **Tauri command pattern** for frontend-backend communication
- Keep the **Attention Score engine** independent and testable
- Preserve the **local-only, privacy-first** design

## 🆘 Getting Help

- Open an issue for bugs or questions
- Check existing issues first
- Be patient with responses — this is a volunteer project
- Provide context and error messages when asking for help

## 📜 License

By contributing, you agree that your contributions will be licensed under the same license as the project.

---

Thank you for contributing to Ants! 🐜
