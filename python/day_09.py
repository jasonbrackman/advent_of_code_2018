from collections import deque, defaultdict


def part01(players, last_marble):
    circle = deque([0])
    current = 0

    totals = defaultdict(int)

    while True:

        for player in range(1, players + 1):
            current += 1
            if len(circle) == 1:
                circle.append(current)
            elif current % 23 == 0:
                totals[player] += current
                for _ in range(6):
                    circle.insert(0, circle.pop())
                keep = circle.pop()
                num = circle.pop()
                totals[player] += num

                circle.append(keep)

            else:
                a = circle.popleft()
                circle.append(a)
                circle.append(current)

            if current == last_marble:
                high = 0
                for k, v in totals.items():
                    if v > high:
                        high = v
                return high


if __name__ == "__main__":
    p1 = part01(459, 71790)
    assert p1 == 386151

    p2 = part01(459, 71790 * 100)
    assert p2 == 3211264152
