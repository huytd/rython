# Color Palette — C64 16-color reference

screen.cls()

colors = [
    "black", "white", "red", "cyan",
    "purple", "green", "blue", "yellow",
    "orange", "brown", "lightred", "darkgray",
    "gray", "lightgreen", "lightblue", "lightgray"
]

screen.text("  C64 Color Palette", 58, 10, 2, "cyan")

x = 10
y = 36
for i, name in enumerate(colors):
    # Colored rectangle swatch
    screen.rect(x, y, 50, 16, name, True)
    # Name label
    screen.text(name, x + 2, y + 2, 1, "white" if name not in ("black", "brown", "darkgray") else "lightgray")
    x += 58
    if (i + 1) % 4 == 0:
        x = 10
        y += 26

# Bottom row — gradient demo
screen.text("  Gradient Demo:", 70, 96, 1, "white")
for x in range(240):
    r = int((x / 240) * 255)
    screen.line(x, 112, x, 130, (r, 255 - r, 128))
