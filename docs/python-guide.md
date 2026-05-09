# pymodo Python API Guide

pymodo gives your Python scripts access to a `screen` object — a 40×25 pixel-art canvas rendered in the terminal. Write code in the built-in editor, press **F5** (or **Ctrl+R**) to run, and see your output immediately.

## Quick Start

```python
screen.clear()
screen.print("Hello, pymodo!", 10, 12, "white", "blue")
for i in range(10):
    screen.set(i, 0, "*", "cyan", "black")
```

Press **F5** to run. The grid shows your result. Press any key to return to the editor.

---

## The `screen` Object

A global object injected into every script. No imports needed.

### Coordinate System

- **40 columns (x)** × **25 rows (y)**
- Origin `(0, 0)` is at the **top-left** corner
- Valid range: `x` = 0–39, `y` = 0–24

```
(0,0) ────────────────> x (39)
  │
  │   y
  ▼ (24)
```

### Available Methods

#### `screen.set(x, y, ch, fg, bg)`

Place a single character at position `(x, y)` with given colors.

| Parameter | Type   | Description                          |
|-----------|--------|--------------------------------------|
| `x`       | int    | Column (0–39)                        |
| `y`       | int    | Row (0–24)                           |
| `ch`      | str    | Single character to display          |
| `fg`      | str    | Foreground color name                |
| `bg`      | str    | Background color name                |

```python
screen.set(10, 5, "A", "yellow", "black")   # Yellow 'A' on black at (10, 5)
screen.set(0, 0, "█", "green", "blue")      # Green block character
```

#### `screen.print(text, x, y, fg, bg)`

Print a string starting at `(x, y)`. Text wraps to the next row when it hits column 40. If it reaches the bottom, the screen scrolls up by one line.

| Parameter | Type   | Description                          |
|-----------|--------|--------------------------------------|
| `text`    | str    | String to print                      |
| `x`       | int    | Starting column (0–39)               |
| `y`       | int    | Starting row (0–24)                  |
| `fg`      | str    | Foreground color name                |
| `bg`      | str    | Background color name                |

```python
screen.print("Hello, world!", 5, 10, "white", "black")
screen.print(f"Score: {42}", 0, 0, "cyan", "black")  # f-strings work too
```

#### `screen.clear()`

Clear the entire grid back to empty cells.

```python
screen.clear()
```

#### `screen.scroll(n)`

Scroll the screen **up** by `n` rows. Bottom rows fill with blanks. If `n >= 25`, the screen is cleared entirely.

```python
screen.scroll(1)   # Scroll up by 1 row
screen.scroll(5)   # Scroll up by 5 rows
```

#### `screen.refresh()`

Force an immediate redraw of the grid. Useful in animation loops so you can see each frame.

```python
for frame in range(30):
    screen.clear()
    screen.set(frame % 40, 12, "@", "green", "black")
    screen.refresh()
```

---

## Colors

Color names are passed as strings to `fg` and `bg` parameters. Case-insensitive.

| Name        | Preview     |
|-------------|-------------|
| `black`     | Black       |
| `white`     | White       |
| `red`       | Red         |
| `cyan`      | Cyan        |
| `purple`    | Purple      |
| `green`     | Green       |
| `blue`      | Blue        |
| `yellow`    | Yellow      |
| `orange`    | Orange      |
| `brown`     | Brown       |
| `lightred`  | Light Red   |
| `darkgray`  | Dark Gray   |
| `gray`      | Gray        |
| `lightgreen`| Light Green |
| `lightblue` | Light Blue  |
| `lightgray` | Light Gray  |

These are C64-inspired RGB values. If an unknown color name is given, a `ValueError` is raised.

```python
screen.print("Rainbow!", 5, 10, "red", "black")
screen.set(20, 5, "O", "LIGHTBLUE", "DARKGRAY")  # Case-insensitive
```

---

## Examples

### Animation

Move a character across the screen with `time.sleep()` and `screen.refresh()`:

```python
import time
for frame in range(30):
    screen.clear()
    x = (frame * 2) % 40
    y = 12
    screen.set(x, y, "@", "green", "black")
    screen.print(f"frame {frame}", 0, 0, "gray", "black")
    screen.refresh()
    time.sleep(0.1)
```

### Color Palette

Show all available colors:

```python
screen.clear()
colors = ["red", "yellow", "green", "cyan", "blue", "purple"]
for i, color in enumerate(colors):
    y = i + 10
    screen.print(f"  {color}", 5, y, color, "black")
```

### Checkerboard Pattern

Draw patterns using nested loops:

```python
screen.clear()
for y in range(20):
    for x in range(40):
        ch = "#" if (x + y) % 2 == 0 else "."
        fg = "yellow" if ch == "#" else "blue"
        screen.set(x, y, ch, fg, "black")
```

---

## Tips and Gotchas

- **Out-of-bounds writes are silently ignored** — `screen.set(50, 5, ...)` does nothing since `x` must be 0–39.
- **`screen.print()` auto-wraps and scrolls** — long strings wrap at column 40; if they reach the bottom row, the screen scrolls up by one.
- **Use `screen.refresh()` in loops** — without it, you only see the final frame after the script finishes.
- **Standard Python works** — `time.sleep()`, f-strings, list comprehensions, etc. are all available. The only special object is `screen`.
- **Errors show on screen** — if your code raises an exception, the error message appears below the grid. Press any key to return to the editor.

---

## Editor Shortcuts

| Key              | Action           |
|------------------|------------------|
| **F5** / Ctrl+R  | Run the script   |
| **Ctrl+O**       | Open a file      |
| **Esc** / Ctrl+C | Quit             |

When running, press any key to return to the editor.
