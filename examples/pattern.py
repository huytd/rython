# Pattern Gallery — Geometric shapes and fills

screen.cls()

# Title
screen.text("  Pattern Gallery", 54, 6, 2, "cyan")

# Filled circle with concentric rings
cx, cy = 40, 50
for r in range(18, 0, -2):
    screen.circ(cx, cy, r, (r * 10, 50, 200), False)

# Checkerboard pattern
for y in range(30, 70):
    for x in range(80, 120):
        if ((x - 80) // 4 + (y - 30) // 4) % 2 == 0:
            screen.pix(x, y, "yellow")
        else:
            screen.pix(x, y, "blue")

# Diagonal lines
for i in range(10):
    screen.line(130 + i * 8, 30, 130 + i * 8, 70, (200 - i * 15, 100, 50))

# Nested rectangles
for i in range(5):
    x = 180 + i * 6
    y = 30 + i * 6
    w = 50 - i * 12
    h = 40 - i * 12
    if w > 0 and h > 0:
        screen.rect(x, y, w, h, (50 + i * 30, 200 - i * 20, 100), False)

# Dashed line
for i in range(0, 240, 8):
    screen.line(i, 90, i + 3, 90, "green")

# Pixel art smiley
for angle in range(0, 360, 5):
    rad = math.radians(angle)
    x = int(120 + 20 * math.cos(rad))
    y = int(108 + 20 * math.sin(rad))
    screen.pix(x, y, "orange")
