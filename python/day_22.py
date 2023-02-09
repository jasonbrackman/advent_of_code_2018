from functools import cache
from typing import Tuple

Pos = Tuple[int, int]

X_MAGIC = 16807
Y_MAGIC = 48271
MOD = 20183

@cache
def erosion_level_index(pos: Pos, target: Pos, depth: int) -> int:
    x, y = pos
    if x == 0 and y == 0:
        return 0
    if x == target[0] and y == target[1]:
        return 0
    if y == 0:
        return (x * X_MAGIC + depth) % MOD
    if x == 0:
        return (y * Y_MAGIC + depth) % MOD

    return (
        erosion_level_index((x - 1, y), target, depth)
        * erosion_level_index((x, y - 1), target, depth)
        + depth
    ) % MOD


def part01() -> int:
    icons = {
        0: ".",
        1: "=",
        2: "|",
    }
    p_depth = 7863
    p_target = 14, 760
    # t_depth = 510
    # t_target = 10, 10
    count = 0
    for y in range(p_target[1] + 1):
        # s = ''
        for x in range(p_target[0] + 1):
            r = erosion_level_index((x, y), p_target, p_depth) % 3
            count += r
            # s += icons[r]
        # print(s)
    return count


if __name__ == "__main__":
    p1 = part01()
    assert p1 == 11462
