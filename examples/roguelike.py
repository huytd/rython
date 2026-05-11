# Roguelike Demo — Dungeon with player movement

import time, random

screen.cls()

# Dungeon dimensions
W, H = 30, 15
OX, OY = 4, 8  # offset on canvas

# Generate dungeon grid
dungeon = []
for y in range(H):
    row = []
    for x in range(W):
        if x == 0 or x == W-1 or y == 0 or y == H-1:
            row.append('#')  # wall
        else:
            row.append('.')  # floor
    dungeon.append(row)

# Place some random walls
for _ in range(40):
    wx = random.randint(2, W-3)
    wy = random.randint(2, H-3)
    dungeon[wy][wx] = '#'

# Place player
px, py = 15, 7
dungeon[py][px] = '@'

# Place items
items = []
for _ in range(8):
    ix, iy = random.randint(1, W-2), random.randint(1, H-2)
    if dungeon[iy][ix] == '.':
        items.append((ix, iy))
        dungeon[iy][ix] = '*'

# Color map
def tile_color(ch):
    if ch == '#': return "gray"
    if ch == '.': return "darkgray"
    if ch == '@': return "green"
    if ch == '*': return "yellow"
    return "white"

def render():
    for y in range(H):
        for x in range(W):
            screen.set(x * 8 + OX, y * 8 + OY, dungeon[y][x], tile_color(dungeon[y][x]), "black")
    screen.text(f"Pos: ({px},{py})", OX, (H+1)*8 + OY, 1, "cyan")

steps = 0
for frame in range(500):
    render()

    # Move player based on input
    key = screen.read_key()
    nx, ny = px, py
    if key == "left" or key == "a": nx -= 1
    elif key == "right" or key == "d": nx += 1
    elif key == "up" or key == "w": ny -= 1
    elif key == "down" or key == "s": ny += 1

    if (nx >= 0 and nx < W and ny >= 0 and ny < H and dungeon[ny][nx] != '#'):
        dungeon[py][px] = '.'
        px, py = nx, ny
        dungeon[py][px] = '@'
        steps += 1

    # Check item pickup
    for i, (ix, iy) in enumerate(items[:]):
        if ix == px and iy == py:
            items.pop(i)
            dungeon[iy][ix] = '.'

    screen.refresh()
    time.sleep(0.08)

screen.cls()
screen.text(f"  Explored {steps} steps!", 48, 60, 1, "green")
