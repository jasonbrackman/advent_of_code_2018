from collections import deque


def parse():
    rules = dict()
    with open(r".././data/day_12.txt") as f:
        state = next(f).split()[2]
        _ = next(f)
        for l in f:
            a, _, c = l.split()
            rules[a] = c

    return state, rules


def part_01(state, rules, r=20):
    seen = set()

    start = 0
    for index in range(r):
        # Check if a stabilized pattern emerges
        s = "".join(state)
        if s in seen:
            # Stabilized -- so calculate offset / and count
            start += r - index
            return count_plants_from_position(start, state)
        else:
            seen.add(s)

        # Otherwise keep mutating
        temp = ""
        state.extend([".", ".", ".", "."])

        new = [".", ".", ".", ".", state.popleft()]
        start -= 2
        while state:
            t = "".join(new)
            rule = rules.get(t, ".")
            temp += rule
            new.pop(0)
            new.append(state.popleft())

        # temp = ''.join(temp)
        length = len(temp)
        temp = temp.lstrip(".")
        length = length - len(temp)
        temp = temp.rstrip(".")
        start += length

        state = deque(list(temp))

    return count_plants_from_position(start, state)


def count_plants_from_position(start, state):
    total = 0
    for c in state:
        if c == "#":
            total += start
        start += 1
    return total


if __name__ == "__main__":
    state, rules = parse()
    state = deque(state)

    p1 = part_01(state, rules)
    assert p1 == 3337

    state, rules = parse()
    state = deque(state)
    p2 = part_01(state, rules, r=50_000_000_000)
    assert p2 == 4300000000349
