# Landscape — Procedural terrain with sky gradient

import time, math, random

screen.cls()

# Sky gradient (top to bottom)
for y in range(50):
    r = int(20 + (y / 50) * 40)
    g = int(20 + (y / 50) * 30)
    b = int(80 + (y / 50) * 100)
    screen.line(0, y, 239, y, (r, g, b))

# Stars
for _ in range(60):
    sx = random.randint(0, 239)
    sy = random.randint(0, 40)
    screen.pix(sx, sy, "white")

# Moon
screen.circ(180, 25, 12, "#dddddd", True)
screen.circ(184, 22, 11, (30, 30, 70), True)  # shadow cutout

# Mountains (layered sine waves)
for layer in range(4):
    base_y = 60 + layer * 15
    amplitude = 18 - layer * 3
    freq = 0.03 + layer * 0.01
    offset = layer * 50
    green = 40 + layer * 25

    for x in range(240):
        y = int(base_y + math.sin((x + offset) * freq) * amplitude
                    + math.sin((x + offset) * freq * 2.3) * (amplitude / 3))
        screen.line(x, y, x, 135, (20, green, 20))

# Water at bottom
for x in range(240):
    for y in range(118, 136):
        wave = int(math.sin(x * 0.1 + y * 0.5) * 10)
        screen.pix(x, y, (20 + wave, 40 + wave, 100))

screen.text("  Landscape", 90, 4, 1, "white")
