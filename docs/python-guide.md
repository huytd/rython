# rython Python API Guide

rython gives your Python scripts access to a `screen` object — a **240×136 pixel-art canvas** rendered in the browser. Write code in the built-in editor, press **▶ Run** (or **Ctrl+Enter**) to execute, and see your output immediately.

## Quick Start

```python
screen.cls()
screen.text("Hello, rython!", 50, 60, 1, "cyan")
for i in range(240):
    screen.pix(i, 100, (i, 255 - i, 128))
```

Press **▶ Run** or **Ctrl+Enter**. The canvas shows your result. Press **■ Stop** to halt execution.

---

## The `screen` Object

A global object injected into every script. No imports needed.

### Coordinate System

- **240 columns (x)** × **136 rows (y)**
- Origin `(0, 0)` is at the **top-left** corner
- Valid range: `x` = 0–239, `y` = 0–135

```
(0,0) ──────────────────────> x (239)
  │
  │   y
  ▼ (135)
```

---

## Pixel Drawing API

### `screen.cls()`

Clear the entire canvas to black.

```python
screen.cls()
```

### `screen.pix(x, y, color)`

Draw a single pixel at `(x, y)` with the given color.

| Parameter | Type   | Description                          |
|-----------|--------|--------------------------------------|
| `x`       | int    | Column (0–239)                       |
| `y`       | int    | Row (0–135)                          |
| `color`   | str or tuple | C64 color name, hex string, or `(r, g, b)` tuple |

```python
screen.pix(120, 68, "red")           # C64 palette name
screen.pix(100, 50, "#ff8800")       # Hex color
screen.pix(150, 80, (255, 136, 0))   # RGB tuple
```

### `screen.get(x, y)`

Read the color of a pixel at `(x, y)`. Returns `[r, g, b]` tuple.

```python
r, g, b = screen.get(120, 68)
```

### `screen.line(x1, y1, x2, y2, color)`

Draw a line from `(x1, y1)` to `(x2, y2)` using Bresenham's algorithm.

```python
screen.line(10, 10, 230, 120, "cyan")
```

### `screen.rect(x, y, w, h, color, filled)`

Draw a rectangle at `(x, y)` with width `w` and height `h`.

| Parameter | Type   | Description                          |
|-----------|--------|--------------------------------------|
| `x`       | int    | Top-left column                      |
| `y`       | int    | Top-left row                         |
| `w`       | int    | Width in pixels                      |
| `h`       | int    | Height in pixels                     |
| `color`   | str or tuple | Fill/stroke color                |
| `filled`  | bool   | `True` to fill, `False` for outline |

```python
screen.rect(50, 30, 100, 60, "green", True)    # Filled rectangle
screen.rect(20, 10, 40, 40, "yellow", False)   # Outline only
```

### `screen.circ(x, y, radius, color, filled)`

Draw a circle centered at `(x, y)` with the given radius.

| Parameter | Type   | Description                          |
|-----------|--------|--------------------------------------|
| `x`       | int    | Center column                        |
| `y`       | int    | Center row                           |
| `radius`  | int    | Radius in pixels                     |
| `color`   | str or tuple | Fill/stroke color                |
| `filled`  | bool   | `True` to fill, `False` for outline |

```python
screen.circ(120, 68, 30, "red", True)       # Filled circle
screen.circ(120, 68, 35, "yellow", False)   # Ring outline
```

### `screen.text(str, x, y, size, color)`

Render text using an embedded 8×8 bitmap font.

| Parameter | Type   | Description                          |
|-----------|--------|--------------------------------------|
| `str`     | str    | Text to render                       |
| `x`       | int    | Starting column                      |
| `y`       | int    | Starting row                         |
| `size`    | int    | Scale factor: 1, 2, 3, or 4          |
| `color`   | str or tuple | Text color                     |

```python
screen.text("Hello!", 50, 60, 2, "cyan")      # Size 2 = 16x16 per char
screen.text("Score: 42", 10, 10, 1, "white")  # Size 1 = 8x8 per char
```

Each character is 8×8 pixels at size 1. At size `N`, each pixel scales to N×N.

### `screen.blit(srcX, srcY, w, h, dstX, dstY)`

Copy a rectangular region of the canvas to another position.

```python
screen.blit(0, 0, 120, 68, 120, 68)   # Mirror top-left to bottom-right
```

---

## Backward-Compatible API

These methods map the original character-grid API onto the pixel canvas for compatibility with legacy scripts.

### `screen.set(x, y, ch, fg, bg)`

Place a single **8×8 bitmap glyph** at `(x, y)` with foreground and background colors. Each "cell" occupies an 8×8 pixel block.

```python
screen.set(10, 5, "A", "yellow", "black")   # Yellow 'A' on black at cell (10, 5)
screen.set(0, 0, "#", "green", "blue")      # Green block character
```

> **Note:** With the pixel grid, `x` = 0–29 (30 cells of 8px each fit in 240px width), `y` = 0–16 (17 cells of 8px each fit in 136px height).

### `screen.print(text, x, y, fg, bg)`

Print text starting at cell `(x, y)`. Auto-wraps and scrolls like the original API.

```python
screen.print("Hello, world!", 5, 10, "white", "black")
```

### `screen.clear()`

Alias for `screen.cls()`.

### `screen.scroll(n)`

Scroll the canvas **up** by `n` pixels. Bottom fills with black.

```python
screen.scroll(8)   # Scroll up by one row of text
```

---

## Colors

### C64 Palette (16 named colors)

| Name        | Preview     | Hex       |
|-------------|-------------|-----------|
| `black`     | Black       | #000000   |
| `white`     | White       | #ffffff   |
| `red`       | Red         | #cc0000   |
| `cyan`      | Cyan        | #66cccc   |
| `purple`    | Purple      | #cc00cc   |
| `green`     | Green       | #00cc00   |
| `blue`      | Blue        | #0000cc   |
| `yellow`    | Yellow      | #cccc00   |
| `orange`    | Orange      | #cc6600   |
| `brown`     | Brown       | #663300   |
| `lightred`  | Light Red   | #ff6666   |
| `darkgray`  | Dark Gray   | #666666   |
| `gray`      | Gray        | #999999   |
| `lightgreen`| Light Green | #66ff66   |
| `lightblue` | Light Blue  | #6699ff   |
| `lightgray` | Light Gray  | #cccccc   |

### Full RGB

Any drawing function also accepts:
- **Hex strings**: `"#ff8800"`, `"#0f3460"`
- **RGB tuples**: `(255, 136, 0)`, `(r, g, b)` where each is 0–255

```python
screen.pix(10, 10, "red")             # C64 name
screen.rect(20, 20, 40, 40, "#ff4400", True)   # Hex
screen.circ(100, 68, 20, (50, 200, 100), True) # RGB tuple
```

---

## Input

### `screen.read_key()`

Read the next keypress from the input queue. Returns `"none"` if no key is available.

```python
key = screen.read_key()
if key == "left":
    x -= 1
elif key == "right":
    x += 1
```

Supported keys: `a`–`z`, `0`–`9`, `up`, `down`, `left`, `right`, `enter`, `esc`, `space`, and more.

### Mouse

```python
x = screen.mouse_x        # Current mouse column (0-239)
y = screen.mouse_y        # Current mouse row (0-135)
down = screen.is_mouse_down()  # True if any mouse button is pressed
```

---

## Canvas Properties

```python
screen.width   # 240
screen.height  # 136
screen.palette()  # List of C64 color name strings
```

---

## Examples

### Animated Sine Wave

```python
import time, math

for t in range(120):
    screen.cls()
    for x in range(240):
        y = int(68 + math.sin(x * 0.05 + t * 0.1) * 30)
        screen.line(x, y, x, y + 1, "#53d769")
    screen.text(f"frame {t}", 4, 4, 1, "white")
    screen.refresh()
    time.sleep(0.02)
```

### Bouncing Ball with Trail

```python
import time, random

x, y = 120, 68
vx, vy = 3, 2

for _ in range(200):
    # Fade previous frame
    for py in range(136):
        for px in range(240):
            r, g, b = screen.get(px, py)
            screen.pix(px, py, (max(0,r-8), max(0,g-8), max(0,b-8)))

    x += vx; y += vy
    if x <= 5 or x >= 234: vx = -vx
    if y <= 5 or y >= 130: vy = -vy

    screen.circ(x, y, 6, "orange", True)
    screen.text(f"({x},{y})", 4, 4, 1, "white")
    screen.refresh()
    time.sleep(0.02)
```

### Fireworks

```python
import time, math, random

particles = []

for frame in range(300):
    # Fade
    for py in range(136):
        for px in range(240):
            r, g, b = screen.get(px, py)
            screen.pix(px, py, (max(0,r-4), max(0,g-4), max(0,b-4)))

    # New explosion every 15 frames
    if frame % 15 == 0:
        ex, ey = random.randint(30,210), random.randint(20,80)
        color = (random.randint(100,255), random.randint(100,255), random.randint(100,255))
        for _ in range(40):
            angle = random.uniform(0, 6.28)
            speed = random.uniform(1, 5)
            particles.append([ex, ey, math.cos(angle)*speed, math.sin(angle)*speed, color, 50])

    # Update & draw
    for p in particles[:]:
        p[0] += p[2]; p[1] += p[3]; p[3] += 0.05; p[5] -= 1
        if p[5] > 0:
            screen.pix(int(p[0]), int(p[1]), p[4])
        else:
            particles.remove(p)

    screen.refresh()
    time.sleep(0.03)
```

---

## Tips and Gotchas

- **Out-of-bounds writes are silently ignored** — `screen.pix(300, 5, ...)` does nothing since the canvas is 240×136.
- **Use `screen.refresh()` in loops** — without it, you only see the final frame after the script finishes.
- **`time.sleep()` auto-refreshes** — calling `time.sleep()` inside a loop triggers an automatic screen refresh, so animations work smoothly.
- **Standard Python works** — `math`, `random`, `time`, f-strings, list comprehensions, classes, etc. are all available via Pyodide.
- **The bitmap font is 8×8 per character** — at size 2 each character is 16×16 pixels, at size 3 it's 24×24, etc.
- **`screen.get()` returns `[r, g, b]`** — useful for pixel manipulation effects like fading, color shifting, and trails.

---

## Editor Shortcuts

| Key              | Action           |
|------------------|------------------|
| **▶ Run** / Ctrl+Enter  | Run the script   |
| **■ Stop**             | Stop execution   |
| **? Help**             | Show API reference |
