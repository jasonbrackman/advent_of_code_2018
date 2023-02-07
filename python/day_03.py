from typing import List, NamedTuple
from collections import namedtuple
import re

pattern = r"#(\d+) @ (\d+),(\d+): (\d+)x(\d+)"


class Swatch(NamedTuple):
    id: int
    left_edge: int
    top_edge: int
    width: int
    height: int


def parse(path: str) -> List[Swatch]:
    swatches = list()
    with open(path, "r") as f:
        items = [line.strip() for line in f]
        for i in items:
            r = re.search(pattern, i)
            if r is not None:
                args = [int(i) for i in r.groups()]
                swatches.append(Swatch(*args))

    return swatches


def get_matrix(swatches: List[Swatch]) -> List[List[int]]:
    matrix = [[0] * 1000 for _ in range(1000)]

    for s in swatches:
        for r in range(s.left_edge, s.left_edge + s.width):
            for c in range(s.top_edge, s.top_edge + s.height):
                matrix[r][c] += 1
    return matrix


def part01(matrix: List[List[int]]) -> int:
    total = 0
    for r in range(1000):
        for c in range(1000):
            if matrix[r][c] > 1:
                total += 1
    return total


def part02(matrix: List[List[int]], swatches: List[Swatch]) -> int:
    for swatch in swatches:
        test = True
        for r in range(swatch.left_edge, swatch.left_edge + swatch.width):
            for c in range(swatch.top_edge, swatch.top_edge + swatch.height):
                if matrix[r][c] > 1:
                    test = False
        if test:
            return swatch.id
    return -1


def run() -> None:
    path = r".././data/day_03.txt"
    swatches = parse(path)
    matrix = get_matrix(swatches)

    p1 = part01(matrix)
    assert p1 == 124850

    p2 = part02(matrix, swatches)
    assert p2 == 1097


if __name__ == "__main__":
    run()
