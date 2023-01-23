from pathlib import Path
from typing import List, Dict, Set, Tuple

from python import helpers

registers = {
    0: 0,
    1: 0,
    2: 0,
    3: 0,
}


class Instruction:
    def __init__(self, op, arg1, arg2, arg3):
        self.op = op
        self.a = arg1
        self.b = arg2
        self.c = arg3


def addr(i: Instruction):
    registers[i.c] = registers[i.a] + registers[i.b]


def addi(i: Instruction):
    registers[i.c] = registers[i.a] + i.b


def mulr(i: Instruction):
    registers[i.c] = registers[i.a] * registers[i.b]


def muli(i: Instruction):
    registers[i.c] = registers[i.a] * i.b


def banr(i: Instruction):
    registers[i.c] = registers[i.a] & registers[i.b]


def bani(i: Instruction):
    registers[i.c] = registers[i.a] & i.b


def borr(i: Instruction):
    registers[i.c] = registers[i.a] | registers[i.b]


def bori(i: Instruction):
    registers[i.c] = registers[i.a] | i.b


def setr(i: Instruction):
    registers[i.c] = registers[i.a]


def seti(i: Instruction):
    registers[i.c] = i.a


def gtir(i: Instruction):
    """(greater-than immediate/register) sets register C to 1 if value A is greater than register B.
    Otherwise, register C is set to 0."""
    registers[i.c] = 1 if i.a > registers[i.b] else 0


def gtri(i: Instruction):
    """(greater-than register/immediate) sets register C to 1 if register A is greater than value B.
    Otherwise, register C is set to 0."""
    registers[i.c] = 1 if registers[i.a] > i.b else 0


def gtrr(i: Instruction):
    """(greater-than register/register) sets register C to 1 if register A is greater than register B.
    Otherwise, register C is set to 0."""
    registers[i.c] = 1 if registers[i.a] > registers[i.b] else 0


def eqir(i: Instruction):
    """(equal immediate/register) sets register C to 1 if value A is equal to register B.
    Otherwise, register C is set to 0."""
    registers[i.c] = 1 if i.a == registers[i.b] else 0


def eqri(i: Instruction):
    """(equal register/immediate) sets register C to 1 if register A is equal to value B.
    Otherwise, register C is set to 0.
    """
    registers[i.c] = 1 if registers[i.a] == i.b else 0


def eqrr(i: Instruction) -> None:
    """(equal register/register) sets register C to 1 if register A is equal to register B.
    Otherwise, register C is set to 0.
    """
    registers[i.c] = 1 if registers[i.a] == registers[i.b] else 0


class OpTest:
    def __init__(self, before, after, data):
        self.before = before
        self.after = after
        self.data = data

    def __repr__(self):
        return f"OpTest({self.before}, {self.after}, {self.data})"


def parse(path: Path) -> Tuple[List[OpTest], List[Instruction]]:
    lines = iter(helpers.lines(path))
    items = []
    codes = []
    while True:
        try:
            a = next(lines)
            if a.startswith("Before"):
                b = next(lines)
                c = next(lines)
                _, before = a.split(':')
                data = Instruction(*[int(i) for i in b.split()])
                _, after = c.split(":")
                items.append(OpTest(eval(before.strip()), eval(after.strip()), data))
                next(lines)
            else:
                if a:
                    codes.append(Instruction(*[int(i) for i in a.split()]))
        except StopIteration:
            break

    return items, codes

ops = {
    0: addr,
    1: addi,
    2: mulr,
    3: muli,
    4: banr,
    5: bani,
    6: borr,
    7: bori,
    8: setr,
    9: seti,
    10: gtir,
    11: gtri,
    12: gtrr,
    13: eqir,
    14: eqri,
    15: eqrr,
}


def part01() -> int:
    path = Path(__file__).parent / '..' / 'data' / 'day_16.txt'
    tests, _ = parse(path)
    winners = 0
    for test in tests:
        t = 0

        for num, cmd in ops.items():
            for index, r in enumerate(test.before):
                registers[index] = r
            cmd(test.data)
            if list(registers.values()) == test.after:
                t += 1

        if t >= 3:
            winners += 1

    return winners


def part02() -> int:
    path = Path(__file__).parent / '..' / 'data' / 'day_16.txt'
    tests, codes = parse(path)
    possibles = _samples(tests)

    lut: Dict[int, int] = {}
    remove: Set[int] = set()
    while len(lut) != 16:
        for k, v in possibles.items():
            v -= remove
            if len(v) == 1:
                lut[k] = list(v)[0]
                remove |= v
    for r in registers:
        registers[r] = 0

    for code in codes:
        ops[lut[code.op]](code)

    return registers[0]


def _samples(tests: List[OpTest]) -> Dict[int, Set[int]]:
    possibles: Dict[int, Set[int]] = {}
    for test in tests:
        t = []

        for num, cmd in ops.items():
            for index, r in enumerate(test.before):
                registers[index] = r
            cmd(test.data)
            if list(registers.values()) == test.after:
                t.append((test.data.op, num))

        for k, v in t:
            if k not in possibles:
                possibles[k] = set()
            possibles[k].add(v)

    return possibles


def run() -> None:
    assert part01() == 605
    assert part02() == 653


if __name__ == "__main__":
    run()
