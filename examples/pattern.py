screen.clear()
for y in range(20):
    for x in range(40):
        ch = "#" if (x + y) % 2 == 0 else "."
        fg = "yellow" if ch == "#" else "blue"
        screen.set(x, y, ch, fg, "black")
