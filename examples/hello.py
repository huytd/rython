# Hello, rython! — Pixel console demo

screen.cls()

# Title text at various sizes
screen.text("  Welcome to rython!", 30, 20, 2, "cyan")
screen.text("  A Retro Python Console", 28, 48, 1, "lightgray")

# Decorative border
screen.rect(0, 0, 240, 136, "blue", False)
screen.rect(1, 1, 238, 134, "blue", False)

# Some colored shapes
screen.circ(50, 100, 12, "green", True)
screen.rect(90, 90, 24, 24, "red", True)
screen.line(130, 80, 160, 110, "yellow")

# Info text
screen.text("  Canvas: 240x136 pixels", 30, 70, 1, "white")
screen.text("  Press Run to re-execute!", 28, 84, 1, "gray")
