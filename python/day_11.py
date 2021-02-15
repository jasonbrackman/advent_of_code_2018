import sys

"""
- Find the fuel cell's rack ID, which is its X coordinate plus 10.
- Begin with a power level of the rack ID times the Y coordinate.
- Increase the power level by the value of the grid serial number (your puzzle input).
- Set the power level to itself multiplied by the rack ID.
- Keep only the hundreds digit of the power level (so 12345 becomes 3; numbers with no hundreds digit become 0).
- Subtract 5 from the power level.
"""
# x = horizontal
# y = vertical
puzzle_input = 1955


def fill_horizontal(grid, dim=3):
    new = []
    for x in grid:
        zippers = [x[i:] for i in range(dim)]
        new.append([sum(group) for group in zip(*zippers)])
    return new


def fill_vertical(grid, dim=3):
    new = []
    for x in range(len(grid)):
        row = []
        xmax = x + dim
        # Skip work if we can
        if xmax < len(grid):
            for y in range(len(grid[0])):
                row.append(sum(grid[item][y] for item in range(x, xmax)))
            new.append(row)

    return new


def initialize_cells():
    cells = [[0] * 300 for _ in range(300)]
    for x in range(len(cells)):
        for y in range(len(cells[0])):
            rack_id = x + 10
            power_level = rack_id * y
            power_level += puzzle_input
            power_level *= rack_id
            power_level = power_level // 10**2 % 10
            power_level -= 5
            cells[x][y] = power_level

    return cells


def part_01(s: int):
    cells = initialize_cells()
    cells = fill_horizontal(cells, dim=s)
    cells = fill_vertical(cells, dim=s)
    m = sys.maxsize * -1
    loc = [None, None]
    for x in range(len(cells)):
        for y in range(len(cells[0])):
            if cells[x][y] > m:
                m = cells[x][y]
                loc = [x, y]
    return m, loc


def part_02():
    m = sys.maxsize * -1
    loc = [None, None]
    idx = sys.maxsize * -1

    for index in range(1, 301):
        s, r = part_01(index)
        if s > m:
            m = s
            loc = r
            idx = index

    return loc[0], loc[1], idx


if __name__ == "__main__":
    _, p1 = part_01(3)
    assert p1 == [21, 93]

    a, b, c = part_02()
    assert f"{a},{b},{c}" == "231,108, 14", (a, b, c)

