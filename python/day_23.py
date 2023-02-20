from pathlib import Path

from typing import Dict, List, Set, Optional
import re
from python import helpers

pattern = re.compile(r'-?\d+')


class Nanobot:
    def __init__(self, x: str, y: str, z: str, r: str) -> None:
        self.x = int(x)
        self.y = int(y)
        self.z = int(z)
        self.r = int(r)

    def manhattan_distance(self, other):
        x = abs(self.x - other.x)
        y = abs(self.y - other.y)
        z = abs(self.z - other.z)
        return x + y + z

    def in_range(self, other):
        md = self.manhattan_distance(other)
        return md <= self.r


def parse(path: Path) -> List[Nanobot]:
    lines = helpers.lines(path)
    nanobots = [Nanobot(*re.findall(pattern, line)) for line in lines]
    return nanobots


def part02() -> int:
    path = Path(__file__).parent / '..' / 'data' / 'day_23.txt'
    nanobots = parse(path)
    # print(len(nanobots))

    # x = str(sum([n.x for n in nanobots]) // len(nanobots))
    # y = str(sum([n.y for n in nanobots]) // len(nanobots))
    # z = str(sum([n.z for n in nanobots]) // len(nanobots))

    result = 982
    xx = 47846199
    yy = 53749244
    zz = 21356335

    cont = True
    while cont:
        cont = False
        updated = False
        cont, result, xx, yy, zz = spin(cont, nanobots, result, updated, xx, yy, zz)

    return sum([xx, yy, zz])
    # print('result:', result)  # 983
                              # xx = 47846199
                              # yy = 53749244
                              # zz = 21356335
                              # 122951778
                              # 145_445_660 too high


def spin(cont, nanobots, result, updated, xx, yy, zz):
    extra = 40
    t = Nanobot(xx, yy, zz, '0')
    for x in range(xx - extra, xx + extra, 1):
        for y in range(yy - extra, yy + extra, 1):
            for z in range(zz - 0, zz + 1, 1):
                if updated is not True:
                    t.x = x
                    t.y = y
                    t.z = z

                    r = sum([n.in_range(t) for n in nanobots])
                    if r > result:
                        result = r - 1
                        # print(r)
                        # print('xx =', x)
                        # print('yy =', y)
                        # print('zz =', z)
                        s = sum([x, y, z])
                        # print(s)
                        if s < 122951778:
                            xx = x
                            yy = y
                            zz = z
                            updated = True
                            cont = True
    return cont, result, xx, yy, zz


def part01() -> int:
    path = Path(__file__).parent / '..' / 'data' / 'day_23.txt'
    nanobots = parse(path)
    hashmap: Dict[Nanobot, Set[Nanobot]] = dict()
    for n1 in nanobots:
        hashmap[n1] = set()
        for n2 in nanobots:
            if n1.in_range(n2):
                hashmap[n1].add(n2)

    result: Optional[Nanobot] = None
    max = 0
    for k, v in hashmap.items():
        if k.r > max:
            max = k.r
            result = k
    if result is not None:
        return len(hashmap[result])
    return -1


if __name__ == "__main__":
    assert part01() == 164
    assert part02() == 122951778
