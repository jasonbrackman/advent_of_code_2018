import sys
from collections import deque
from string import ascii_lowercase


path = r".././data/day_05.txt"
with open(path, "r") as f:
    polymer = f.read().strip()


def get_span(polymer):
    s = list()
    p = deque(polymer)

    while p:
        if len(p) == 1:
            s.append(p.popleft())
        else:
            a = p.popleft()
            b = p.popleft()
            if a.lower() == b.lower() and a != b:
                pass
            else:
                s.append(a)
                p.extendleft(b)

    return "".join(s)


def part01(polymer):
    count = len(polymer)
    while True:
        polymer = get_span(polymer)
        if len(polymer) == count:
            return polymer
        count = len(polymer)


def part02(polymer):
    low = sys.maxsize

    for char in ascii_lowercase:
        if char in polymer:
            result = polymer.replace(char, "").replace(char.upper(), "")
            new = part01(result)
            if len(new) < low:
                low = len(new)

    return low


if __name__ == "__main__":
    p1 = part01(polymer)
    assert len(p1) == 11668

    p2 = part02(polymer)
    assert p2 == 4652
