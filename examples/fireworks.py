# Fireworks — Particle explosion demo

import time, math, random

screen.cls()

class Particle:
    def __init__(self, x, y, color):
        self.x = x
        self.y = y
        angle = random.uniform(0, 6.28)
        speed = random.uniform(1, 5)
        self.vx = math.cos(angle) * speed
        self.vy = math.sin(angle) * speed
        self.color = color
        self.life = random.randint(30, 70)

    def update(self):
        self.x += self.vx
        self.y += self.vy
        self.vy += 0.05  # gravity
        self.life -= 1

    def draw(self):
        if self.life > 0:
            r, g, b = self.color
            alpha = int(self.life / 70 * 255)
            screen.pix(int(self.x), int(self.y), (r, g, b))

def explode(x, y):
    colors = [(255,100,50), (255,200,50), (100,255,100), (100,150,255), (255,100,255)]
    color = random.choice(colors)
    for _ in range(40):
        particles.append(Particle(x, y, color))

particles = []

for frame in range(300):
    # Fade background
    for py in range(136):
        for px in range(240):
            r, g, b = screen.get(px, py)
            screen.pix(px, py, (max(0, r - 4), max(0, g - 4), max(0, b - 4)))

    # Launch new firework every 15 frames
    if frame % 15 == 0:
        explode(random.randint(30, 210), random.randint(20, 80))

    # Update and draw particles
    for p in particles[:]:
        p.update()
        if p.life <= 0:
            particles.remove(p)
        else:
            p.draw()

    screen.text(f"fireworks", 96, 4, 1, "yellow")
    screen.refresh()
    time.sleep(0.03)
