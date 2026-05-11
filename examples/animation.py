# Animation — Bouncing ball with trail effect

import time, math

screen.cls()

x, y = 120, 68
vx, vy = 3, 2
trail = []

for frame in range(200):
    # Fade previous frame (darken all pixels slightly)
    for py in range(136):
        for px in range(240):
            r, g, b = screen.get(px, py)
            screen.pix(px, py, (max(0, r - 8), max(0, g - 8), max(0, b - 8)))

    # Move ball
    x += vx
    y += vy
    if x <= 5 or x >= 234: vx = -vx
    if y <= 5 or y >= 130: vy = -vy

    # Draw ball with glow
    screen.circ(x, y, 8, "darkgray", True)
    screen.circ(x, y, 6, "orange", True)
    screen.circ(x, y, 4, "yellow", True)

    screen.text(f"frame {frame}", 4, 4, 1, "white")
    screen.refresh()
    time.sleep(0.02)
