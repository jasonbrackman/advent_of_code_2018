from pathlib import Path
from typing import List, Tuple

from python import helpers


class Code:
    def __init__(self, op: str, arg1: int, arg2: int, arg3: int) -> None:
        self.op = op
        self.a = arg1
        self.b = arg2
        self.c = arg3

    def __str__(self) -> str:
        return f"{self.op} {self.a} {self.b} {self.c}"


class Frame:
    def __init__(self, code: Code, rv1: str, op: str, rv2: str, message: str = ""):
        self.code = code
        self.rv1 = rv1
        self.op = op
        self.rv2 = rv2
        self.message = message


class Machine:
    DEBUG = True

    def __init__(self, ip: int, codes: List[Code]) -> None:
        self.ip = ip
        self.codes = codes
        self.count = 0
        self.registers = {
            0: 0,
            1: 0,
            2: 0,
            3: 0,
            4: 0,
            5: 0,
        }

    def run(self) -> int:
        while self.registers[self.ip] < len(self.codes):
            code = self.codes[self.registers[self.ip]]
            eval(f"self.{code.op}")(code)

            self.registers[self.ip] += 1
            self.count += 1

        return self.registers[0]

    def debug(self, frame: Frame) -> None:
        if Machine.DEBUG:

            print(
                f"{self.count:05} | {self.registers[self.ip]:02} | {frame.code.op} | R{frame.code.c} = {frame.rv1}{frame.code.a} {frame.op:<2} {frame.rv2}{frame.code.b} | {list(self.registers.values())} | {frame.message}"
            )

    def addr(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] + self.registers[code.b]
        msg = "Important: Increase of R0!" if code.c == 0 else ''
        self.debug(Frame(code, "R", "+", "R", message=msg))

    def addi(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] + code.b
        msg = "Important: Increase of R0!" if code.c == 0 else ''
        self.debug(Frame(code, "R", "+", "V", message=msg))

    def mulr(self, code: Code):
        self.registers[code.c] = self.registers[code.a] * self.registers[code.b]
        self.debug(Frame(code, "R", "*", "R"))

    def muli(self, code: Code):
        self.registers[code.c] = self.registers[code.a] * code.b
        self.debug(Frame(code, "R", "*", "V"))

    def setr(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a]
        self.debug(Frame(code, "R", '=', "!"))

    def seti(self, code: Code) -> None:
        self.registers[code.c] = code.a
        self.debug(Frame(code, "V", '=', "!"))

    def gtrr(self, code: Code) -> None:
        """(greater-than register/register) sets register C to 1 if register A is greater than register B.
        Otherwise, register C is set to 0."""
        self.registers[code.c] = 1 if self.registers[code.a] > self.registers[code.b] else 0
        self.debug(Frame(code, "R", '>', "R", message="If R2 > R1 increase R1; else ..."))

    def eqrr(self, code: Code) -> None:
        """(equal register/register) sets register C to 1 if register A is equal to register B.
        Otherwise, register C is set to 0.
        """
        self.registers[code.c] = (
            1 if self.registers[code.a] == self.registers[code.b] else 0
        )
        self.debug(Frame(code, "R", "==", "R", message="IF R3 * R2 == R1 increase R4 to 1 else 0"))


def parse(path: Path) -> Tuple[int, List[Code]]:
    lines = iter(helpers.lines(path))
    ip = int(next(lines).split()[1])
    codes = []
    for line in lines:
        cmd, a, b, c = line.split()
        codes.append(Code(cmd, int(a), int(b), int(c)))

    return ip, codes


def part01() -> int:
    path = Path(__file__).parent / ".." / "data" / "day_19.txt"
    ip, codes = parse(path)
    machine = Machine(ip, codes)
    Machine.DEBUG = False
    return machine.run()


def part02() -> int:
    # path = Path(__file__).parent / ".." / "data" / "day_19.txt"
    # ip, codes = parse(path)
    # machine = Machine(ip, codes)
    # machine.registers[0] = 1  # change from part01
    # return machine.run()

    # wow... walking through some debugging statements it felt like the largest number in R1
    # was being checked for all of its multiples.  Then just sum() them together.  A quick
    # Python snippit proved it worked with p1 -- so let p2 run for a few moments to get the
    # following number:
    num = 10551267
    t = set()
    for x in range(1, num + 1):
        if num % x == 0:
            t.add(x)
    return sum(t)


def run() -> None:
    assert part01() == 1228
    assert part02() == 15285504


if __name__ == "__main__":
    run()
