screen.clear()
colors = ["red", "yellow", "green", "cyan", "blue", "purple"]
for i, color in enumerate(colors):
    y = i + 10
    screen.print(f"  {color}", 5, y, color, "black")
