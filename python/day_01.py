from typing import List
from itertools import cycle
from collections import defaultdict


def part01(items: List[int]) -> int:
    return sum(items)


def part02(items: List[int]) -> int:
    # look for first repeated result

    results = defaultdict(int)

    current = 0
    for i in cycle(items):
        current += i
        if results[current] == 1:
            return current
        results[current] += 1


if __name__ == "__main__":
    path = r".././data/day_01.txt"
    with open(path, "r") as f:
        items = [int(i) for i in f.readlines()]

    p1 = part01(items)
    p2 = part02(items)
    assert p1 == 533
    assert p2 == 73272
