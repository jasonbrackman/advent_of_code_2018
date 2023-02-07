from functools import cache
from typing import Tuple, List

import helpers
from pathlib import Path

"""
RULES:
An open acre will become filled with trees if three or more adjacent acres contained trees. 
    Otherwise, nothing happens.
An acre filled with trees will become a lumberyard if three or more adjacent acres were lumberyards. 
    Otherwise, nothing happens.
An acre containing a lumberyard will remain a lumberyard if it was adjacent to at least one other lumberyard and 
at least one acre containing trees. 
    Otherwise, it becomes open.
"""


GROUND = "."
TREE = "|"
LUMBERYARD = "#"

Pos = Tuple[int, int]
Data = List[List[str]]


def aura(pos: Pos, data: Data) -> str:
    y, x = pos
    target = data[y][x]
    items = {
        GROUND: 0,
        TREE: 0,
        LUMBERYARD: 0,
    }
    for y_ in (y-1, y, y+1):
        if 0 <= y_ < len(data):
            for x_ in (x-1, x, x+1):
                if 0 <= x_ < len(data[0]):
                    if (y_, x_) != (y, x):
                        items[data[y_][x_]] += 1

    if target == GROUND:
        return TREE if items[TREE] >= 3 else GROUND

    elif target == TREE:
        return LUMBERYARD if items[LUMBERYARD] >= 3 else TREE

    # elif target == LUMBERYARD:
    # Else Target Must Be LUMBERYARD
    return LUMBERYARD if items[LUMBERYARD] >= 1 and items[TREE] >= 1 else GROUND


def parse(path: Path) -> List[List[str]]:
    lines = helpers.lines(path)
    return [list(line) for line in lines]


def spin(data: List[List[str]]) -> List[List[str]]:
    new_data = []
    for y in range(len(data)):
        nd = []
        for x in range(len(data[0])):
            nd.append(aura((y, x), data))
        new_data.append(nd)
    return new_data


def part_01(data: List[List[str]]) -> int:
    for c in range(10):
        data = spin(data)

    tree_count = 0
    lumb_count = 0
    for row in data:
        for r in row:
            tree_count += (r == TREE)
            lumb_count += (r == LUMBERYARD)
    # print(tree_count, "*", lumb_count, '=', tree_count * lumb_count)

    return tree_count * lumb_count


def part_02(data: List[List[str]]) -> int:
    results = []
    for c in range(1, 1_000_000_000+1):
        data = spin(data)
        if str(data) not in results:
            results.append(str(data))
        else:
            repeat_start = results.index(str(data))
            # print('Found Repeat:', c, repeat_start)
            # looks like repeat starts at 537 ... matching 508 ... 535 (28 frames then cycle)

            remainder = (1_000_000_000 - c) % 28
            data_rows: List[str] = list(results[repeat_start + remainder])

            tree_count = 0
            lumb_count = 0
            for row in data_rows:
                for r in row:
                    tree_count += r == TREE
                    lumb_count += r == LUMBERYARD
            # r = tree_count, lumb_count
            return tree_count * lumb_count
            # print(tree_count, "*", lumb_count, '=', tree_count * lumb_count)
    return -1


def run() -> None:
    path = Path(__file__).parent / '.' / '..' / 'data' / 'day_18.txt'
    data = parse(path)
    assert part_01(data) == 360720
    assert part_02(data) == 197276


if __name__ == "__main__":
    run()
