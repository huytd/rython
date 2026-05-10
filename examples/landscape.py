"""Animated night-to-day landscape with twinkling stars, mountains, trees, water, and a moving sun.
Press Q to quit."""
import time
import math
import random

W, H = 60, 24

# --- Pre-generate star positions (upper half) ---
stars = [(random.randint(0, W - 1), random.randint(0, 7)) for _ in range(50)]
star_phases = [random.random() * 2 * math.pi for _ in stars]

# --- Mountain peaks (x, height) ---
mountains = [
    (8, 10), (18, 8), (30, 12), (42, 9), (52, 11),
]

# --- Tree positions ---
trees = [(5, 16), (12, 17), (25, 15), (35, 18), (48, 16), (55, 17)]

# --- Cloud definitions: (x, y, width) ---
clouds = [(10, 3, 6), (30, 5, 8), (48, 2, 5)]


def mountain_char(dist_from_peak, peak_height, row_offset):
    """Return a mountain character based on distance from the peak."""
    if dist_from_peak == 0:
        return "^"
    elif dist_from_peak <= 1:
        return "/" if True else "\\"
    elif dist_from_peak <= 2:
        return "h"
    elif dist_from_peak <= 3:
        return "H"
    elif dist_from_peak <= 4:
        return "#"
    return ""


def render(t):
    screen.clear()

    # --- sky gradient (dawn/dusk cycle over 120 frames) ---
    cycle = (t % 120) / 120.0  # 0=midnight, 0.5=noon, 1.0=midnight
    if cycle < 0.25:
        sky_fg, sky_bg = "lightgray", "black"            # dawn approaching
    elif cycle < 0.5:
        sky_fg, sky_bg = "white", "blue"                  # day
    elif cycle < 0.75:
        sky_fg, sky_bg = "yellow", "purple"               # sunset
    else:
        sky_fg, sky_bg = "lightgray", "black"             # night

    for y in range(14):
        for x in range(W):
            screen.set(x, y, " ", sky_fg, sky_bg)

    # --- stars (visible at night) ---
    show_stars = cycle < 0.2 or cycle > 0.8
    if show_stars:
        brightness = min(1.0, abs(cycle - 0.5) * 4)
        for i, (sx, sy) in enumerate(stars):
            twinkle = math.sin(t * 0.3 + star_phases[i]) * 0.5 + 0.5
            if twinkle > 0.6:
                ch = "*" if brightness > 0.5 else "."
                fg = "yellow" if twinkle > 0.85 else "lightgray"
                screen.set(sx, sy, ch, fg, sky_bg)

    # --- clouds ---
    for cx, cy, cw in clouds:
        cloud_x = (cx + t * 2) % (W + cw) - cw // 2
        for dx in range(cw):
            if 0 <= cloud_x + dx < W:
                ch = "=" if dx == 0 or dx == cw - 1 else "-"
                screen.set(cloud_x + dx, cy, ch, "gray", sky_bg)

    # --- sun/moon arc ---
    celestial_x = int((t % 60) / 60 * (W - 4)) + 2
    if cycle < 0.5:  # sun during day
        celestial_y = 3
        celestial_ch = "O"
        celestial_fg = "yellow"
    else:  # moon at night
        celestial_y = 2
        celestial_ch = "o"
        celestial_fg = "lightgray"

    if 0 <= celestial_x < W:
        screen.set(celestial_x, celestial_y, celestial_ch, celestial_fg, sky_bg)
        # glow effect
        for gx in range(max(0, celestial_x - 1), min(W, celestial_x + 2)):
            for gy in range(max(0, celestial_y - 1), min(14, celestial_y + 2)):
                if (gx, gy) != (celestial_x, celestial_y):
                    dist = abs(gx - celestial_x) + abs(gy - celestial_y)
                    glow_ch = "." if dist == 1 else " "
                    screen.set(gx, gy, glow_ch, celestial_fg, sky_bg)

    # --- mountains ---
    ground_start = 14
    for mx, mh in mountains:
        for dy in range(mh):
            row = ground_start - dy
            if row < 0 or row >= H:
                continue
            peak_row = ground_start - mh + 1
            base_width = max(2, mh // 2)
            for dx in range(-base_width, base_width + 1):
                x = mx + dx
                if x < 0 or x >= W or row < 0 or row >= H:
                    continue
                dist_from_peak = abs(dx)
                slope = (ground_start - row) / mh  # 0=base, 1=peak

                if dist_from_peak > base_width * slope + 0.5:
                    continue

                if row == peak_row and dx == 0:
                    ch, fg = "^", "white"          # snow cap
                elif slope > 0.7:
                    ch, fg = "/", "lightgray"
                elif slope > 0.4:
                    ch, fg = "#", "darkgray"
                else:
                    ch, fg = "H", "brown"

                screen.set(x, row, ch, fg, "black")

    # --- ground and trees ---
    for y in range(ground_start, H - 3):
        for x in range(W):
            if y == ground_start:
                screen.set(x, y, "+", "green", "black")
            else:
                grass = [".", ":", "'", ""][random.randint(0, 3)]
                screen.set(x, y, grass, "darkgray", "black")

    # --- trees ---
    for tx, ty in trees:
        if tx < W and ty < H:
            # trunk
            screen.set(tx, ty + 1, "|", "brown", "black")
            screen.set(tx, ty, "|", "brown", "black")
            # foliage
            for dy in range(-2, 0):
                row = ty + dy
                if row < 0:
                    continue
                spread = 2 if dy == -2 else 1
                for dx in range(-spread, spread + 1):
                    x = tx + dx
                    if 0 <= x < W and 0 <= row < H:
                        screen.set(x, row, "*", "green", "black")

    # --- water with animated waves (bottom 2 rows) ---
    water_rows = [H - 3, H - 2]
    for wr in water_rows:
        for x in range(W):
            wave = math.sin(t * 0.15 + x * 0.4)
            if wave > 0.3:
                ch = "~"
                fg = "lightblue"
            elif wave > -0.3:
                ch = "-"
                fg = "blue"
            else:
                ch = "~"
                fg = "darkgray"
            screen.set(x, wr, ch, fg, "black")

    # --- reflection shimmer in water ---
    if cycle < 0.5:
        for x in range(0, W, 3):
            shimmer = math.sin(t * 0.2 + x) * 0.5 + 0.5
            if shimmer > 0.7:
                screen.set(x, H - 3, "'", "lightblue", "black")

    # --- HUD ---
    phase = ["Night", "Dawn", "Day", "Sunset"][int(cycle * 4) % 4]
    hud = f" Phase: {phase}   Frame: {t}   Q=Quit"
    screen.print(hud, max(0, W - len(hud)), H - 1, "yellow", "black")


frame = 0
while True:
    key = screen.read_key()
    if key in ("q", "Q"):
        break
    render(frame)
    frame += 1
    time.sleep(0.08)
