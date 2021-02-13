import re
import sys
from collections import namedtuple
from copy import deepcopy

pattern = re.compile(r'(-?\d+)')

Pos = namedtuple("Pos", "x, y")
Vel = namedtuple("Vel", "x, y")


def parse():
    pos = []
    vel = []
    with open(r'.././data/day_10.txt') as f:
        for line in f:
            p1, p2, v1, v2 = re.findall(pattern, line)
            p = Pos(int(p1), int(p2))
            v = Vel(int(v1), int(v2))
            pos.append(p)
            vel.append(v)
    return pos, vel


def grid(rows, cols):
    return [['.'] * rows for _ in range(cols)]


def part01(grid, new, offset):
    for n in new:
        grid[n.y-offset][n.x-offset] = '#'
    for g in grid:
        print(' '.join(g))


def get_row_col_and_offsets(pos):

    cols = [p.x for p in pos]
    rows = [p.y for p in pos]
    cmin, col = min(cols), max(cols)
    rmin, row = min(rows), max(rows)
    cols_offset = abs(cmin)
    rows_offset = abs(rmin)

    return col, row, cols_offset, rows_offset


if __name__ == "__main__":
    pos, vel = parse()
    col, row, cols_offset, rows_offset = get_row_col_and_offsets(pos)
    start = [Pos(p.x + cols_offset, p.y + rows_offset) for p in pos]
    size = [col + cols_offset, row + rows_offset]

    for count in range(1, sys.maxsize):
        new = []
        for i in range(len(start)):
            new.append(Pos(start[i].x + vel[i].x, start[i].y + vel[i].y))
        col, row, cols_offset, rows_offset = get_row_col_and_offsets(new)

        new_size = [col + cols_offset, row + rows_offset]

        if size[0] < new_size[0] or size[1] < new_size[1]:
            print(f"A winner in `{count}` seconds")
            # Part 2 answer
            assert count == 10081

            grid = grid(col - cols_offset + 1, row - rows_offset + 1)
            part01(grid, new, cols_offset)

            break

        size = new_size
        start = new





