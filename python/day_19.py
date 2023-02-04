from pathlib import Path
from typing import List, Tuple

from python import helpers

registers = {
    0: 0,
    1: 0,
    2: 0,
    3: 0,
    4: 0,
    5: 0,
}


class Instruction:
    def __init__(self, op: str, arg1: int, arg2: int, arg3: int) -> None:
        self.op = op
        self.a = arg1
        self.b = arg2
        self.c = arg3

    def __str__(self) -> str:
        return f"{self.op} {self.a} {self.b} {self.c}"

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


def parse(path: Path) -> Tuple[int, List[Instruction]]:
    lines = iter(helpers.lines(path))
    ip = int(next(lines).split()[1])
    codes = []
    for line in lines:
        cmd, a,b, c = line.split()
        codes.append(Instruction(cmd, int(a), int(b), int(c)))

    return ip, codes


def part01() -> int:
    path = Path(__file__).parent / '..' / 'data' / 'day_19.txt'
    ip, instructions = parse(path)

    while registers[ip] < len(instructions):
        code = instructions[registers[ip]]
        eval(code.op)(code)
        registers[ip] += 1
    return registers[0]


# registers = {
#         0: 1,
#         1: 0,
#         2: 0,
#         3: 0,
#         4: 0,
#         5: 0,
#     }
#
#
# def part02() -> int:
#     path = Path(__file__).parent / '..' / 'data' / 'day_19.txt'
#     ip, instructions = parse(path)
#
#     while registers[ip] < len(instructions):
#         print(registers)
#         code = instructions[registers[ip]]
#         eval(code.op)(code)
#         registers[ip] += 1
#
#     return registers[0]
#

def run() -> None:
    assert part01() == 1228
    # p2 = part02()
    # assert p2 == 1228, f"You got: {p2}"


if __name__ == "__main__":
    run()