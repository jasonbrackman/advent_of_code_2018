from collections import defaultdict
import re


class Guard:
    def __init__(self, id, date, hour, minute):
        self.id = id
        self.date = date
        self.hour = hour
        self.minute = minute
        self.asleep = defaultdict(int)
        self.count = 0

    def sleep(self, start, end):
        for i in range(start, end):
            self.count += 1
            self.asleep[i] += 1


compile01 = re.compile(r'\[(\d+)-(\d+)-(\d+) (\d+):(\d+)] \w+ (\w+|#\w+)')


def get_sorted_data(path):
    guards = list()
    with open(path, 'r') as f:

        lines = [line.strip() for line in f]
        for line in lines:
            r = re.search(compile01, line)
            year, month, day, *nums, last = r.groups()
            date = int(year + month + day)
            items = [date] + [int(num) for num in nums]
            if last.startswith('#'):
                items.append(int(last[1:]))
            else:
                items.append(last)

            guards.append(items)
    return sorted(guards)


def get_schedule(data):
    current = None
    start = 0

    guards = list()
    for d in data:

        if type(d[-1]) == str:
            if d[-1] == 'asleep':
                start = d[2]
            if d[-1] == 'up':
                end = d[2]
                current.sleep(start, end)
        else:
            if current and current not in guards:
                guards.append(current)

            current = None
            for guard in guards:
                if guard.id == d[3]:
                    current = guard

            if current is None:
                current = Guard(id=d[3], date=d[0], hour=d[1], minute=d[2])

    return guards


def part01(guards):
    guard_to_focus_on = None
    high = 0
    for g in guards:
        if g.count > high:
            high = g.count
            guard_to_focus_on = g

    solution = 0
    high = 0
    for k, v in guard_to_focus_on.asleep.items():
        if v > high:
            high = v
            solution = k

    return guard_to_focus_on.id * solution


def part02(guards) -> int:
    _id = None
    minute = None
    high = 0
    for g in guards:
        for k, v in g.asleep.items():
            if v > high:
                high = v
                minute = k
                _id = g.id

    return _id * minute


if __name__ == "__main__":
    path = r'.././data/day_04.txt'
    data = get_sorted_data(path)
    guards = get_schedule(data)

    p1 = part01(guards)
    assert p1 == 140932

    p2 = part02(guards)
    assert p2 == 51232

