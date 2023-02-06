from collections import namedtuple
from typing import List, Set, Dict

import helpers
from pathlib import Path

"""find the number of constellations of points in the list.

Two points are in the same constellation if their manhattan distance apart is no more than 3 
or if they can form a chain of points, each a manhattan distance no more than 3 from the last, 
between the two of them. 
(That is, if a point is close enough to a constellation, it "joins" that constellation.)"""

Point = namedtuple('Point', 'x, y, z, t')


def manhattan_distance(p1: Point, p2: Point) -> int:
    x: int = abs(p1.x - p2.x)
    y: int = abs(p1.y - p2.y)
    z: int = abs(p1.z - p2.z)
    t: int = abs(p1.t - p2.t)
    return y + x + z + t


def parse(path: Path) -> List[Point]:
    lines = helpers.lines(path)
    points = []
    for line in lines:
        points.append(Point(*[int(i) for i in line.split(",")]))
    return points


def test01() -> None:
    path = Path(__file__).parent / '..' / 'data' / 'day_25_test.txt'
    points = parse(path)
    connections = create_connections(points)
    assert unique_connection_count(connections) == 2


def test02() -> None:
    path = Path(__file__).parent / '..' / 'data' / 'day_25_test2.txt'
    points = parse(path)
    connections = create_connections(points)
    assert unique_connection_count(connections) == 4


def test03() -> None:
    path = Path(__file__).parent / '..' / 'data' / 'day_25_test3.txt'
    points = parse(path)
    connections = create_connections(points)
    assert unique_connection_count(connections) == 8


def test04() -> None:
    path = Path(__file__).parent / '..' / 'data' / 'day_25_test4.txt'
    points = parse(path)
    connections = create_connections(points)
    x = unique_connection_count(connections)
    assert x == 3, f'got {x}'


def part01() -> int:
    path = Path(__file__).parent / '..' / 'data' / 'day_25.txt'
    points = parse(path)
    connections = create_connections(points)
    return unique_connection_count(connections)


def unique_connection_count(connections):

    seen = set()

    count = 0
    for k, v in connections.items():
        temps = {k, } | v
        for k2, v2 in connections.items():
            temps2 = {k2, } | v2
            for t in temps:
                if t in temps2:
                    connections[k2] |= temps
                    connections[k] |= temps2

    for k, v in connections.items():
        if any(test in seen for test in ({k, } | v)):
            continue
        seen.add(k)
        seen |= v
        # print('Bucket:', k, v)
        count += 1
    return count


def create_connections(points: List[Point]) -> Dict[Point, Set[Point]]:
    connections = dict()
    for p1 in points:
        connections[p1] = set()
        for p2 in points:
            r = manhattan_distance(p1, p2)
            if 0 < r <= 3:
                connections[p1].add(p2)

    return connections

def run() -> None:
    assert part01() == 394

if __name__ == "__main__":
    run()
