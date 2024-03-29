
from typing import Tuple, Dict, Iterable
from collections import deque

Grid = Dict[Tuple[int, int], str]
Vec2 = Tuple[int, int]

dirs: Dict[str, Tuple[int, int]] = {
    "N": (0, -1),  # x, y OR col, row
    "E": (1, 0),
    "S": (0, 1),
    "W": (-1, 0),
}


def update_pos(p1: Vec2, p2: Vec2) -> Vec2:
    return p1[0] + p2[0], p1[1] + p2[1]


def create_grid(chars: str) -> Grid:
    pos = 0, 0

    grid: Grid = dict()
    grid[pos] = "X"
    stack = []

    for c in chars:
        if c in "^$":
            # ignore start/end items
            pass
        elif c in "(":
            # hold the current position in the stack
            stack.append(pos)
        elif c == ")":
            # current position should perma-backtrack
            pos = stack.pop()
        elif c == "|":
            # current position should backtrack
            pos = stack[-1]
        else:
            # update position
            pos = update_pos(pos, dirs[c])
            if pos not in grid:
                grid[pos] = "|" if c in "EW" else "-"
            pos = update_pos(pos, dirs[c])
            if pos not in grid:
                grid[pos] = "."
    return grid


def node_neighbours(grid: Grid, node: Vec2) -> Iterable[Vec2]:
    for k, v in dirs.items():
        new = update_pos(node, v)
        if new in grid and grid[new] in (".", "|", "-"):
            yield new


def dfs(grid: Grid, goal: Vec2) -> int:
    start = (0, 0)
    visited = {
        start,
    }
    q = deque([(start, 0)])
    while q:
        node, door_count = q.popleft()
        if node == goal:
            return door_count

        for neighbour in node_neighbours(grid, node):
            if neighbour not in visited:
                visited.add(neighbour)
                q.append((neighbour, door_count + 1))

    raise ValueError("An impossible problem was provided.")


def parse() -> str:
    with open(r"../data/day_20.txt", "r", encoding="utf-8") as handle:
        return handle.read().strip()


def run() -> None:
    chars = parse()
    grid = create_grid(chars)
    above_999 = 0
    result_door = 0.0
    for pos, icon in grid.items():
        if icon == ".":
            result = dfs(grid, pos)
            above_999 += result > 1999  # twice the amount
            if result > result_door:
                result_door = result

    assert result_door / 2 == 3675.0
    assert above_999 == 7953


if __name__ == "__main__":
    run()
