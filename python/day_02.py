from collections import Counter
from typing import List


def part01(items: List[str]) -> int:
    twos = 0
    threes = 0

    for s in items:
        _two = 0
        _thr = 0
        for k, v in Counter(s).items():
            if v == 3:
                _thr += 1
            if v == 2:
                _two += 1
        twos += 0 if _two == 0 else 1
        threes += 0 if _thr == 0 else 1

    return twos * threes


def part02(items: List[str]) -> str:
    while items:
        a = items.pop()
        for b in items:
            compare = [int(a1 != a2) for a1, a2 in zip(a, b)]
            if sum(compare) == 1:
                final = [a[i] for i, c in enumerate(compare) if c == 0]
                return str(''.join(final))


if __name__ == "__main__":
    path = r'.././data/day_02.txt'
    with open(path, 'r') as f:
        items = [i.strip() for i in f]

    p1 = part01(items)
    assert p1 == 4980

    p2 = part02(items)
    assert p2 == "qysdtrkloagnfozuwujmhrbvx"
