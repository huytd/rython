# Minimal Roguelike — collect all coins and escape!
# Arrow keys or WASD to move. R to restart. Q to quit.

W, H = 40, 25
WALL, FLOOR, COIN, PLAYER, EXIT = '#', '.', '$', '@', '>',

# --- level layout ---
map_data = [
  "########################################",
  "#                                      #",
  "#  #######    ########   ########      #",
  "#  #     #    #      #   #        #    #",
  "#  #  $  #    #  $   #   #   $    #    #",
  "#  #######    ########   ########      #",
  "#                                      #",
  "#  #######    ########   ########      #",
  "#  #     #    #      #   #        #    #",
  "#  #     #    #  $   #   #   $    #    #",
  "#  #  $  #    #      #   #        #    #",
  "#  #######    ########   ########      #",
  "#                                      #",
  "#  #######    ########   ########      #",
  "#  #     #    #      #   #        #    #",
  "#  #  $  #    #      #   #    $   #    #",
  "#  #     #    #  $   #   #        #    #",
  "#  #######    ########   ########      #",
  "#                                      #",
  "#          ###############             #",
  "#          #              #            #",
  "#          #       $      #            #",
  "#          #     >        #            #",
  "#          ###############             #",
  "########################################",
]

# --- parse map ---
walls = set()
coins_template = set()
for r, row in enumerate(map_data):
    for c, ch in enumerate(row):
        if ch == WALL: walls.add((c, r))
        elif ch == COIN: coins_template.add((c, r))

# game state
player_x, player_y = 1, 1
coins = set(coins_template)
score = 0
total_coins = len(coins)
message = "Collect all coins and reach the exit!"
game_over = False
win = False
turns = 0

KEYS_UP    = {"up", "w", "k"}
KEYS_DOWN  = {"down", "s", "j"}
KEYS_LEFT  = {"left", "a", "h"}
KEYS_RIGHT = {"right", "d", "l"}


def render():
    screen.clear()

    for x in range(W):
        for y in range(H):
            ch, fg = ' ', 'black'
            if (x, y) == (player_x, player_y):
                ch, fg = PLAYER, 'lightgreen'
            elif (x, y) in coins:
                ch, fg = COIN, 'yellow'
            elif exit_pos and (x, y) == exit_pos:
                ch, fg = EXIT, 'cyan'
            elif (x, y) in walls:
                ch, fg = WALL, 'gray'
            else:
                ch, fg = FLOOR, 'darkgray'
            screen.set(x, y, ch, fg, 'black')

    # HUD
    hud = f"Score: {score}/{total_coins}  Turns: {turns}  "
    if game_over:
        hud += "YOU WIN! Press R or Q." if win else "GAME OVER. Press R or Q."
    else:
        hud += message

    screen.print(hud, 0, H - 1, 'white', 'blue')
    screen.print("Arrow/WASD: Move  |  R: Restart  |  Q: Quit", 0, H - 2, 'gray', 'black')
    screen.refresh()


def reset():
    global player_x, player_y, coins, score, message, game_over, win, turns, exit_pos
    player_x, player_y = 1, 1
    coins = set(coins_template)
    score = 0
    turns = 0
    message = "Collect all coins and reach the exit!"
    game_over = False
    win = False
    exit_pos = None


exit_pos = None

while True:
    key = screen.read_key()

    if key in ("q", "Q"):
        break
    if key in ("r", "R") and game_over:
        reset()
        render()
        continue

    if not game_over:
        dx, dy = 0, 0
        if key in KEYS_UP:    dy = -1
        elif key in KEYS_DOWN: dy = 1
        elif key in KEYS_LEFT: dx = -1
        elif key in KEYS_RIGHT: dx = 1

        if dx or dy:
            nx, ny = player_x + dx, player_y + dy
            if (nx, ny) not in walls and 0 <= nx < W and 0 <= ny < H:
                player_x, player_y = nx, ny
                turns += 1

                if (player_x, player_y) in coins:
                    coins.discard((player_x, player_y))
                    score += 1
                    message = f"+1 coin! ({score}/{total_coins})"

                if not coins and exit_pos is None:
                    exit_pos = (20, 22)
                    message = "Exit revealed! Find the > symbol!"

                if exit_pos and (player_x, player_y) == exit_pos:
                    game_over = True
                    win = True
                    message = f"Escaped in {turns} turns!"
            else:
                message = "Bump! Can't go there."

    render()
