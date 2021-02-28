from sys import maxsize
from collections import deque


def parse():
    with open(r".././data/day_14.txt") as handle:
        data = handle.read().strip()
    return data


def part01(puzzle):

    elves = [0, 1]
    scores = deque([3, 7])
    for _ in range(maxsize):
        recipe = sum(scores[x % len(scores)] for x in elves)
        scores.extend([int(i) for i in str(recipe)])
        for index in range(len(elves)):
            elves[index] = (
                elves[index] + scores[elves[index] % len(scores)] + 1
            ) % len(scores)

        if len(scores) == puzzle + 10:
            return "".join([str(scores[i]) for i in range(puzzle, puzzle + 10)])


def part02(puzzle):

    puzzle = [int(i) for i in puzzle]

    left = 0
    right = len(puzzle)

    elves = [0, 1]
    scores = [3, 7]

    while True:
        recipe = sum(scores[x % len(scores)] for x in elves)
        scores.extend([int(i) for i in str(recipe)])

        for idx in [0, 1]:
            elves[idx] = (elves[idx] + scores[elves[idx] % len(scores)] + 1) % len(
                scores
            )

        if scores[left:right] == puzzle:
            return len(scores[0:left])

        left += 1
        right += 1


if __name__ == "__main__":
    p1_input = int(parse())
    p1 = part01(p1_input)
    assert p1 == "2688510125"

    p2_input = parse()
    p2 = part02(p2_input)
    assert p2 == 20188250
