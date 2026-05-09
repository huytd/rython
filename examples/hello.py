screen.clear()
screen.print("Hello, pymodo!", 10, 12, "white", "blue")
for i in range(10):
    screen.set(i, 0, "*", "cyan", "black")
