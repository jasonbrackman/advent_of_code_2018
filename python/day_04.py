from collections import defaultdict
import re
from typing import List, Union, Optional, Dict


class Guard:
    def __init__(self, id_: int, date: int, hour: int, minute: int) -> None:
        self.id = id_
        self.date = date
        self.hour = hour
        self.minute = minute
        self.asleep: Dict[int, int] = defaultdict(int)
        self.count = 0

    def sleep(self, start: int, end: int) -> None:
        for i in range(start, end):
            self.count += 1
            self.asleep[i] += 1


compile01 = re.compile(r"\[(\d+)-(\d+)-(\d+) (\d+):(\d+)] \w+ (\w+|#\w+)")


def get_sorted_data(path: str) -> List[Union[int, str]]:
    guards = []
    with open(path, "r") as f:
        lines = [line.strip() for line in f]
        for line in lines:
            r = re.search(compile01, line)
            if r:
                year, month, day, *nums, last = r.groups()
                date = int(year + month + day)
                last = int(last[1:]) if last.startswith("#") else last
                items = [date] + [int(num) for num in nums] + [last]

                guards.append(items)

    return sorted(guards)


def get_schedule(data) -> List[Guard]:
    current: Optional[Guard] = None
    start = 0

    guards: List[Guard] = []
    for d in data:

        if type(d[-1]) == str:
            if d[-1] == "asleep":
                start = d[2]

            if current and d[-1] == "up":
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
                current = Guard(id_=d[3], date=d[0], hour=d[1], minute=d[2])

    return guards


def part01(guards: List[Guard]) -> int:
    guard_highest: Optional[Guard] = None
    solution = 0

    high = 0
    for g in guards:
        if g.count > high:
            high = g.count
            guard_highest = g

    if guard_highest is not None:

        high = 0
        for k, v in guard_highest.asleep.items():
            if v > high:
                high = v
                solution = k

    if isinstance(guard_highest, Guard):
        return guard_highest.id * solution

    return -1


def part02(guards: List[Guard]) -> int:
    _id = -1
    minute = -1
    high = 0
    for g in guards:
        for k, v in g.asleep.items():
            if v > high:
                high = v
                minute = k
                _id = g.id

    return _id * minute


def run() -> None:
    path = r".././data/day_04.txt"
    data = get_sorted_data(path)
    guards = get_schedule(data)

    p1 = part01(guards)
    assert p1 == 140932

    p2 = part02(guards)
    assert p2 == 51232


if __name__ == "__main__":
    run()
