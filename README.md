# GUI Helper

A Rust CLI tool that provides hierarchical grid overlays for AI agents to interact with GUI applications precisely.

## Features

- **Two-Level Grid System**: Overview grid covering the entire screen, plus zoom grids for precise targeting
- **Screenshot Capture**: Automatic screenshot capture with grid overlays
- **Mouse Click Automation**: Precise clicking at calculated grid coordinates  
- **Session Management**: Persistent state across multi-step workflows
- **JSON Export**: Machine-readable coordinate data for AI agent consumption
- **Cross-Platform**: Works on Linux, macOS, and Windows

## Installation

```bash
# Clone the repository
git clone https://github.com/yourusername/gui-helper.git
cd gui-helper

# Build the project
cargo build --release

# The binary will be available at ./target/release/gui-helper
```

## Usage

### Basic Workflow

1. **Show Overview Grid**: Display a full-screen grid with alphanumeric IDs
```bash
gui-helper overview --duration 3
```

2. **Zoom Into Specific Area**: Focus on a selected square with fine-grained numeric grid
```bash
gui-helper zoom --square B5 --duration 2
```

3. **Click Precise Location**: Perform automated click within the zoom area
```bash
gui-helper click --zoom 23
```

### Commands

#### `overview` - Full-Screen Grid
```bash
gui-helper overview [OPTIONS]
  --grid-size <SIZE>     Override grid square size (default: 100px)
  --duration <SECONDS>   How long to show overlay (default: 3s)
  --output <FILE>        Custom screenshot filename
  --json                 Export grid coordinates to JSON
```

#### `zoom` - Zoom Grid
```bash
gui-helper zoom --square <ID> [OPTIONS]
  --square <ID>          Required: square ID from overview (e.g., "B5")
  --padding <PIXELS>     Padding around selected square (default: 50px)
  --subdivision <N>      Grid subdivision NxN (default: 10x10)
  --duration <SECONDS>   Overlay duration
  --output <FILE>        Screenshot filename
  --json                 Export zoom coordinates
```

#### `click` - Automated Clicking
```bash
gui-helper click --zoom <NUMBER> [OPTIONS]
  --zoom <NUMBER>        Required: zoom grid square number (1-100)
  --delay <MS>          Click delay in milliseconds (default: 100)
  --double              Perform double-click
```

#### `open` - View Screenshots
```bash
gui-helper open --latest           # Open latest screenshot
gui-helper open <path>             # Open specific screenshot
```

#### `session` - Session Management
```bash
gui-helper session                 # Show current session
gui-helper session --clear         # Clear session data
```

#### `config` - Configuration
```bash
gui-helper config --show           # Show current config
gui-helper config --reset          # Reset to defaults
```

## Configuration

The tool uses a TOML configuration file located at:
- Linux: `~/.config/gui-helper/config.toml`
- macOS: `~/Library/Application Support/gui-helper/config.toml` 
- Windows: `%APPDATA%\gui-helper\config.toml`

### Example Configuration
```toml
[overview_grid]
square_size = 100
color = "red"
thickness = 2
opacity = 0.7
show_numbers = true
font_size = 16

[zoom_grid]
padding = 50
subdivision = 10
color = "blue"
thickness = 1
opacity = 0.8
show_numbers = true
font_size = 12

[display]
duration = 3
output_format = "png"
save_screenshots = true
screenshot_dir = "./screenshots"

[automation]
click_delay = 100
double_click = false
```

## Data Storage

- **Screenshots**: `~/.local/share/gui-helper/screenshots/`
- **Session Data**: `~/.local/share/gui-helper/session.json`
- **Configuration**: `~/.config/gui-helper/config.toml`

## JSON Export Format

When using `--json`, coordinate data is exported for programmatic use:

```json
{
  "overview_grid": [
    {
      "id": "A1",
      "numeric_id": 0,
      "row": 0,
      "col": 0,
      "x": 0,
      "y": 0,
      "width": 100,
      "height": 100,
      "center_x": 50,
      "center_y": 50
    }
  ],
  "screen_info": {
    "width": 1920,
    "height": 1080,
    "grid_size": 100,
    "rows": 10,
    "cols": 19
  },
  "timestamp": "2025-09-03T10:43:49.123Z"
}
```

## AI Agent Integration

The tool is designed for AI agents to:

1. **Analyze screenshots** with overlay grids to understand GUI layout
2. **Parse JSON coordinate data** to calculate precise click locations
3. **Chain commands** using session management for complex interactions
4. **Adapt to different screen sizes** with automatic grid generation

## Development

```bash
# Run in development mode
cargo run -- overview --duration 1

# Run tests
cargo test

# Build for release
cargo build --release
```

## License

MIT License - see [LICENSE](LICENSE) file for details.

## Contributing

Contributions welcome! Please feel free to submit a Pull Request.