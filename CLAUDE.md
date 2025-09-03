# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Commands

### Development
```bash
# Run in development mode
cargo run -- overview --duration 1

# Build for release
cargo build --release

# Run tests
cargo test
```

### Usage Examples
```bash
# Show full-screen grid overlay for 3 seconds
gui-helper overview --duration 3

# Zoom into a specific square with fine grid
gui-helper zoom --square B5 --duration 2

# Perform automated click within zoom area
gui-helper click --zoom 23

# Maximize the currently active window
gui-helper maximize

# Restore window from maximized state (unmaximize)
gui-helper unmaximize

# View latest screenshot
gui-helper open --latest

# Show current session state
gui-helper session

# Show configuration
gui-helper config --show
```

## Architecture

This is a Rust CLI tool that provides hierarchical grid overlays for AI agent GUI automation. The architecture consists of:

### Core Modules
- **commands/**: Command implementations for each CLI subcommand (overview, zoom, click, maximize, unmaximize, config, session, open)
- **grid/**: Data structures for grid coordinates and screen areas (`OverviewSquare`, `ZoomSquare`, `ZoomArea`, `ScreenInfo`)
- **overlay/**: GUI overlay rendering system with window management and drawing
- **screenshot/**: Screen capture functionality for creating annotated screenshots
- **automation/**: Mouse click automation using the `enigo` crate, and window management using `wmctrl` on Linux
- **session/**: Persistent state management for multi-step workflows (expires after 1 hour)
- **config/**: TOML configuration management with platform-specific paths

### Two-Level Grid System
The tool implements a hierarchical approach:
1. **Overview Grid**: Full-screen grid with alphanumeric IDs (A1, B5, etc.)
2. **Zoom Grid**: Fine-grained numeric grid (1-100) within selected overview squares

### Data Flow
1. Overview command captures screen, generates grid, optionally exports JSON coordinates
2. Zoom command uses session data from overview to create focused grid overlay
3. Click command calculates precise coordinates from zoom grid for mouse automation
4. Session management persists state between commands for chained operations

### Key Dependencies
- `clap`: CLI argument parsing with derive macros
- `screenshots`: Cross-platform screen capture
- `image`: Image processing and manipulation
- `enigo`: Mouse automation
- `serde`/`serde_json`: Serialization for session data and JSON export
- `toml`: Configuration file parsing
- `dirs`: Platform-specific directory paths

### Configuration Locations
- Linux: `~/.config/gui-helper/config.toml`
- macOS: `~/Library/Application Support/gui-helper/config.toml`
- Windows: `%APPDATA%\gui-helper\config.toml`

Session data and screenshots are stored in platform-specific data directories under `gui-helper/`.
- Focus on development for Ubuntu Linux 22.04/24.04