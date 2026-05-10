"""Fireworks particle display with gravity, trails, and color palettes.
Press Q to quit."""
import time
import math
import random

W, H = 60, 24

PARTICLES = []
PALETTES = [
    ["yellow", "orange", "red"],
    ["cyan", "lightblue", "white"],
    ["green", "lightgreen", "yellow"],
    ["purple", "pink" if False else "lightred", "white"],
    ["orange", "yellow", "lightgray"],
]

CHARS = ["*", "+", ".", "'", "o", "@"]


def launch():
    """Launch a new firework from the bottom."""
    x = random.randint(5, W - 6)
    palette = random.choice(PALETTES)
    char = random.choice(CHARS)
    target_y = random.randint(4, 10)

    # Rising rocket particles
    for _ in range(random.randint(3, 7)):
        PARTICLES.append({
            "x": x + random.uniform(-0.5, 0.5),
            "y": H - 2,
            "vx": random.uniform(-0.1, 0.1),
            "vy": -random.uniform(1.5, 3.0),
            "life": 1.0,
            "decay": 0.04,
            "color": palette[0],
            "ch": char,
            "target_y": target_y,
            "exploded": False,
        })


def explode(x, y, palette, char):
    """Create an explosion of particles at (x, y)."""
    count = random.randint(20, 40)
    for _ in range(count):
        angle = random.uniform(0, 2 * math.pi)
        speed = random.uniform(0.3, 2.5)
        color = random.choice(palette)
        PARTICLES.append({
            "x": x,
            "y": y,
            "vx": math.cos(angle) * speed,
            "vy": math.sin(angle) * speed,
            "life": 1.0,
            "decay": random.uniform(0.015, 0.035),
            "color": color,
            "ch": char,
            "target_y": None,
            "exploded": True,
        })


def render(frame):
    screen.clear()

    # --- starry background ---
    for y in range(H - 2):
        for x in range(W):
            if random.random() < 0.03:
                screen.set(x, y, ".", "darkgray", "black")
            else:
                screen.set(x, y, " ", "white", "black")

    # --- ground ---
    for x in range(W):
        screen.set(x, H - 2, "+", "green", "black")
        if random.random() < 0.3:
            screen.set(x, H - 1, "*", "darkgray", "black")
        else:
            screen.set(x, H - 1, " ", "white", "black")

    # --- update and draw particles ---
    surviving = []
    for p in PARTICLES:
        if p["life"] <= 0:
            continue

        p["x"] += p["vx"]
        p["y"] += p["vy"]
        p["vy"] += 0.05  # gravity
        p["life"] -= p["decay"]

        # Check if rocket reached target
        if not p["exploded"] and p["target_y"] is not None:
            if p["y"] <= p["target_y"]:
                palette = random.choice(PALETTES)
                explode(p["x"], p["y"], palette, random.choice(CHARS))
                continue

        px = int(round(p["x"]))
        py = int(round(p["y"]))

        if 0 <= px < W and 0 <= py < H - 1:
            # Trail effect (dimmer copy behind)
            trail_x = int(round(p["x"] - p["vx"] * 2))
            trail_y = int(round(p["y"] - p["vy"] * 2))
            if 0 <= trail_x < W and 0 <= trail_y < H - 1 and p["life"] > 0.3:
                screen.set(trail_x, trail_y, ".", "darkgray", "black")

            alpha = max(0, min(1, p["life"]))
            ch = p["ch"] if alpha > 0.4 else "."
            screen.set(px, py, ch, p["color"], "black")

        surviving.append(p)

    PARTICLES.clear()
    PARTICLES.extend(surviving)

    # --- HUD ---
    hud = f" Particles: {len(PARTICLES)}   Q=Quit"
    screen.print(hud, max(0, W - len(hud)), H - 1, "yellow", "black")


# --- main loop ---
frame = 0
launch_timer = 0

while True:
    key = screen.read_key()
    if key in ("q", "Q"):
        break

    launch_timer += 1
    # Launch new fireworks periodically or randomly
    if launch_timer % 30 == 0 or random.random() < 0.05:
        launch()
        launch_timer = 0

    render(frame)
    frame += 1
    time.sleep(0.06)
