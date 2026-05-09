import time
for frame in range(30):
    screen.clear()
    x = (frame * 2) % 40
    y = 12
    screen.set(x, y, "@", "green", "black")
    screen.print(f"frame {frame}", 0, 0, "gray", "black")
    screen.refresh()
    time.sleep(0.1)
