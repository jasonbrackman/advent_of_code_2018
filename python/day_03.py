from typing import List
from collections import namedtuple
import re

pattern = r'#(\d+) @ (\d+),(\d+): (\d+)x(\d+)'
n = namedtuple("swatch", "id, left_edge, top_edge, width, height")


def parse(path):
    swatches = list()
    with open(path, 'r') as f:
        items = [line.strip() for line in f]
        for i in items:
            r = re.search(pattern, i)
            args = [int(i) for i in r.groups()]
            swatches.append(n(*args))

    return swatches


def get_matrix(swatches):
    matrix = [[0] * 1000 for _ in range(1000)]

    for s in swatches:
        for r in range(s.left_edge, s.left_edge + s.width):
            for c in range(s.top_edge, s.top_edge + s.height):
                matrix[r][c] += 1
    return matrix


def part01(matrix):
    total = 0
    for r in range(1000):
        for c in range(1000):
            if matrix[r][c] > 1:
                total += 1
    return total


def part02(matrix, swatches):

    for s in swatches:
        test = True
        for r in range(s.left_edge, s.left_edge + s.width):
            for c in range(s.top_edge, s.top_edge + s.height):
                if matrix[r][c] > 1:
                    test = False
        if test:
            return s.id


if __name__ == "__main__":
    path = r'.././data/day_03.txt'
    swatches = parse(path)
    matrix = get_matrix(swatches)

    p1 = part01(matrix)
    assert p1 == 124850

    p2 = part02(matrix, swatches)
    assert p2 == 1097

