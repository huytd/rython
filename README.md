# rython — Retro Python Console

A browser-based fantasy console for creative coding in Python. Inspired by TIC-80, PICO-8, and the C64 era.

Write Python code → see pixel-art output instantly. No setup, no installs — just open `index.html` in a browser.

## Features

- **240×136 pixel canvas** with crisp pixelated rendering
- **Python in the browser** via Pyodide (WASM) — full standard library
- **Pixel drawing API**: points, lines, rectangles, circles, text, blitting
- **C64 color palette** + full RGB hex/tuple support
- **Built-in code editor** with syntax highlighting (CodeMirror 6)
- **Bitmap font** for retro text rendering at 4 scale sizes
- **Keyboard & mouse input** from Python scripts
- **Backward-compatible** with the original terminal-based `screen` API

## Quick Start

1. Open `index.html` in any modern browser (Chrome, Firefox, Edge, Safari)
2. Wait for Pyodide to load (~5 seconds on first open)
3. Edit the Python code in the left panel
4. Press **▶ Run** or **Ctrl+Enter** to execute
5. See your pixel art appear on the right canvas

## Screen API

```python
# Pixel drawing
screen.cls()                           # Clear canvas
screen.pix(x, y, color)                # Draw a pixel
screen.line(x1, y1, x2, y2, color)    # Draw a line
screen.rect(x, y, w, h, color, fill)  # Draw a rectangle
screen.circ(x, y, r, color, fill)     # Draw a circle
screen.text(str, x, y, size, color)   # Render bitmap text
screen.blit(sx, sy, w, h, dx, dy)     # Copy pixel region

# Read pixels
r, g, b = screen.get(x, y)            # Get pixel color

# Legacy API (8×8 glyph cells)
screen.set(x, y, ch, fg, bg)          # Place a character cell
screen.print(text, x, y, fg, bg)      # Print text with auto-wrap

# Input
key = screen.read_key()               # Read keyboard input
x = screen.mouse_x                     # Mouse position
y = screen.mouse_y
down = screen.is_mouse_down()         # Mouse button state

# Properties
screen.width    # 240
screen.height   # 136
screen.palette()  # ["black", "white", "red", ...]
```

### Colors

- **C64 palette**: `black`, `white`, `red`, `cyan`, `purple`, `green`, `blue`, `yellow`, `orange`, `brown`, `lightred`, `darkgray`, `gray`, `lightgreen`, `lightblue`, `lightgray`
- **Hex**: `"#ff8800"`
- **RGB tuple**: `(255, 136, 0)`

## Examples

| File | Description |
|------|-------------|
| [`hello.py`](examples/hello.py) | Welcome screen with shapes and text |
| [`colors.py`](examples/colors.py) | C64 palette reference + gradient demo |
| [`animation.py`](examples/animation.py) | Bouncing ball with trail effect |
| [`fireworks.py`](examples/fireworks.py) | Particle explosion simulation |
| [`landscape.py`](examples/landscape.py) | Procedural terrain with sky and water |
| [`pattern.py`](examples/pattern.py) | Geometric shapes and fills gallery |
| [`roguelike.py`](examples/roguelike.py) | Dungeon exploration with keyboard input |

## Architecture

```
┌─────────────────────────────────────────────────────┐
│                    Browser                          │
│                                                     │
│  ┌──────────────┐    ┌──────────────────────────┐   │
│  │  CodeMirror  │    │   Canvas (240×136)       │   │
│  │   Editor     │    │   pixelated rendering     │   │
│  │              │    │                          │   │
│  │  Python code │───>│   screen API (JS)        │   │
│  │              │    │   └─ bitmap font         │   │
│  └──────────────┘    │   └─ C64 palette         │   │
│                      │                          │   │
│  ┌──────────────────┐ │                          │   │
│  │   Pyodide (WASM) │ │                          │   │
│  │   Python runtime │ │                          │   │
│  └──────────────────┘ │                          │   │
│                       └──────────────────────────┘   │
└─────────────────────────────────────────────────────┘
```

- **Single file**: `index.html` — no build step, no server required
- **Pyodide**: Full CPython compiled to WebAssembly (~10MB download)
- **CodeMirror 6**: Syntax-highlighted editor with Python language support
- **Canvas 2D**: Direct pixel manipulation via `ImageData` for maximum performance

## Development

The original Rust-based terminal app source remains in `src/`. The new web version is a standalone `index.html` that can be opened directly in any browser.

### File Structure

```
rython/
├── index.html              # Web console (single-file app)
├── examples/               # Python demo scripts
│   ├── hello.py
│   ├── colors.py
│   ├── animation.py
│   ├── fireworks.py
│   ├── landscape.py
│   ├── pattern.py
│   └── roguelike.py
├── docs/
│   └── python-guide.md     # Full API reference
├── src/                    # Original Rust terminal app (legacy)
│   ├── main.rs
│   ├── grid/
│   ├── tui/
│   ├── python/
│   └── event.rs
└── Cargo.toml              # Rust project config (legacy)
```

## Tips

- **Animations**: Use `time.sleep()` in loops — it auto-refreshes the screen
- **Performance**: For fast animations, minimize per-frame pixel reads (`screen.get()`)
- **Text sizing**: Size 1 = 8×8px/char, Size 2 = 16×16px, up to Size 4 = 32×32px
- **Trail effects**: Fade each frame by reading pixels and darkening them slightly
