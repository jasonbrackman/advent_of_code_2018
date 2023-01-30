import re
from collections import defaultdict
from pathlib import Path
from typing import Set, Dict, Tuple

import helpers

PATTERN = re.compile(r"\d+")


Board = Dict[int, Set[int]]
Pos = Tuple[int, int]


def parse(path) -> Board:
    lines = helpers.lines(path)

    board = defaultdict(set)
    for line in lines:
        t = line[0]
        pat = re.findall(PATTERN, line)
        if t == "x":
            x, y1, y2 = [int(i) for i in pat]
            for y in range(y1, y2 + 1):
                board[y].add(x)
        else:
            y, x1, x2 = [int(i) for i in pat]
            for x in range(x1, x2 + 1):
                board[y].add(x)

    return board


def drip(board: Board):
    start_pos = (0, 500)
    min_y = min(board.keys())
    max_y = max(board.keys())
    water: Set[Pos] = set()
    items: Set[Pos] = set()

    result = -1
    q = [start_pos]
    while q:
        q.sort()
        drip_pos = q.pop(0)
        items.add(drip_pos)

        if drip_pos[0] >= max_y:
            r = len({item for item in water | items if min_y <= item[0] <= max_y})
            if r > result:
                result = r
            continue

        next_pos = drip_pos[0] + 1, drip_pos[1]
        next_is_floor = drip_pos[1] in board[drip_pos[0] + 1] or next_pos in water

        if next_is_floor:
            # Go horizontal
            is_water, data = get_possibles(board, drip_pos, water)
            if is_water:
                water |= data
                # reset drip
                if start_pos not in q:
                    q.append(start_pos)

            else:
                for d in data:
                    if d not in q:
                        y, x = d
                        if x not in board[y + 1] and (y + 1, x) not in water:
                            # print("adding:", d)
                            q.append(d)
                items |= data

        else:
            if next_pos not in q and next_pos not in water:
                q.append(next_pos)

    return result, len(water)


def get_possibles(board, drip_pos, water):
    y_pos = drip_pos[0]
    y_down = drip_pos[0] + 1
    x_clays = board[y_pos]
    x_left = drip_pos[1] - 1
    x_right = drip_pos[1] + 1

    left_continue = True
    right_continue = True
    possibles = {
        drip_pos,
    }
    drops = set()
    while left_continue or right_continue:
        if left_continue:
            if x_left not in x_clays:
                if x_left in board[y_down] or (y_down, x_left) in water:
                    possibles.add((y_pos, x_left))
                else:
                    drops.add((y_pos, x_left))
                    drops.add((y_down, x_left))
                    left_continue = False
            else:
                left_continue = False
        x_left -= 1

        if right_continue:
            if x_right not in x_clays:

                if x_right in board[y_down] or (y_down, x_right) in water:
                    possibles.add((y_pos, x_right))
                else:
                    drops.add((y_pos, x_right))
                    drops.add((y_down, x_right))
                    right_continue = False
            else:
                right_continue = False
        x_right += 1

    if not drops:
        return True, possibles
    else:
        return False, drops | possibles

    return False, set()


def pprint(board, water, items):
    x_min = sorted(water | items, key=lambda x: x[1])[0][1]
    x_max = sorted(water | items, key=lambda x: x[1])[-1][1]
    for y in range(0, max(board.keys()) + 1):
        s = ""
        for x in range(x_min - 5, x_max + 5):
            if x in board[y]:
                s += "#"
            elif (y, x) == (0, 500):
                s += "+"
            elif (y, x) in water:
                s += "~"
                count += 1
            elif (y, x) in items:
                s += "|"
                count += 1
            else:
                s += " "
        print(s)


def run() -> None:
    path = Path(__file__).parent / ".." / "data" / "day_17.txt"

    # Setup vars
    board = parse(path)
    p1, p2 = drip(board)

    assert p1 == 33052
    assert p2 == 27068


if __name__ == "__main__":
    run()
