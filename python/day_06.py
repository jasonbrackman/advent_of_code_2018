from string import ascii_uppercase, ascii_lowercase
from collections import namedtuple, defaultdict

pos = namedtuple("Pos", "col row")
a_z = list(ascii_uppercase) + list(ascii_lowercase)
path = r".././data/day_06.txt"


def get_positions(path: str) -> dict:
    positions = dict()
    with open(path, "r") as f:
        coordinates = [line.strip().split(",") for line in f]
        for (col, row) in coordinates:
            positions[pos(int(row), int(col))] = a_z.pop(0)
    return positions


def get_shortest_distance(positions, t):
    results = defaultdict(list)
    for p, n in positions.items():
        results[manhattan_distance(p, t)].append(n)

    key = sorted(results.keys())[0]
    return results[key][0] if len(results[key]) == 1 else "."


def get_distance_le(board, positions, less=0):
    good = 0
    for row in range(len(board)):
        for col in range(len(board[row])):
            s = sum(manhattan_distance(pos(row, col), p) for p in positions)
            good += s < less
    return good


def set_board(positions: dict):
    cols = [p.col for p, k in positions.items()]
    rows = [p.row for p, k in positions.items()]

    offset = min(cols) if min(cols) > min(rows) else min(rows)

    board = [["."] * (max(cols) + offset) for _ in range(max(rows) + offset)]

    for i in range(min(rows) - offset, max(rows) + offset):
        for j in range(min(cols) - offset, max(cols) + offset):
            t = pos(i, j)
            if t in positions:
                board[i][j] = positions[t]
            else:
                board[i][j] = get_shortest_distance(positions, t)

    return board


def display(board):
    for b in board:
        print("".join(b))


def part01(board, positions):
    result = -1
    for k, v in positions.items():
        c = [c == v for r in board for c in r]
        if sum(c) > result:
            result = sum(c)
            winner = v
    return result


def manhattan_distance(p1, p2):
    return abs(p1.row - p2.row) + abs(p1.col - p2.col)


def bounded_positions(board, positions):

    cs = set(board[0]) | set(board[-1])
    for b in board:
        cs.add(b[0])
        cs.add(b[-1])
    return {p: v for p, v in positions.items() if v not in cs}


if __name__ == "__main__":
    positions = get_positions(path)
    board = set_board(positions)
    bound_positions = bounded_positions(board, positions)

    p1 = part01(board, bound_positions)
    assert p1 == 2342
    p2 = get_distance_le(board, positions, less=10_000)
    assert p2 == 43302
