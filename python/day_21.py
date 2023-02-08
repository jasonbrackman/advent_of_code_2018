from pathlib import Path
from typing import List, Tuple, Dict

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
    def __init__(self, registers: Dict[int, int], code: Code, rv1: str, op: str, rv2: str, message: str = ""):
        self.registers = registers
        self.code = code
        self.rv1 = rv1
        self.op = op
        self.rv2 = rv2
        self.message = message

    def __str__(self) -> str:
        return (
                f"| {self.registers[5]:02} | {self.code.op} | R{self.code.c} = {self.rv1}{self.code.a} {self.op:<2} {self.rv2}{self.code.b} | {list(self.registers.values())} | {self.message}"
            )


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
            if 1_500 < self.count <= 2_500:
                print(self.count, end='')
                print(frame)
            # print(
            #     f"{self.count:05} | {self.registers[self.ip]:02} | {frame.code.op} | R{frame.code.c} = {frame.rv1}{frame.code.a} {frame.op:<2} {frame.rv2}{frame.code.b} | {list(self.registers.values())} | {frame.message}"
            # )

    def addr(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] + self.registers[code.b]
        msg = "Important: Increase of R0!" if code.c == 0 else ""
        self.debug(Frame(self.registers, code, "R", "+", "R", message=msg))

    def addi(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] + code.b
        msg = "Important: Increase of R0!" if code.c == 0 else ""
        self.debug(Frame(self.registers, code, "R", "+", "V", message=msg))

    def banr(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] & self.registers[code.b]
        self.debug(Frame(self.registers, code, "R", "&", "R"))

    def bani(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] & code.b
        self.debug(Frame(self.registers, code, "R", "&", "v"))

    def borr(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] | self.registers[code.b]
        self.debug(Frame(self.registers, code, "R", "|", "R"))

    def bori(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] | code.b
        self.debug(Frame(self.registers, code, "R", "|", "V"))

    def mulr(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] * self.registers[code.b]
        self.debug(Frame(self.registers, code, "R", "*", "R"))

    def muli(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a] * code.b
        self.debug(Frame(self.registers, code, "R", "*", "V"))

    def setr(self, code: Code) -> None:
        self.registers[code.c] = self.registers[code.a]
        self.debug(Frame(self.registers, code, "R", "=", "!"))

    def seti(self, code: Code) -> None:
        self.registers[code.c] = code.a
        self.debug(Frame(self.registers, code, "V", "=", "!"))

    def gtir(self, code: Code) -> None:
        """(greater-than immediate/register) sets register C to 1 if value A is greater than register B.
        Otherwise, register C is set to 0."""
        self.registers[code.c] = 1 if code.a > self.registers[code.b] else 0
        self.debug(Frame(self.registers, code, "V", ">", "R"))

    def gtri(self, code: Code) -> None:
        """(greater-than register/immediate) sets register C to 1 if register A is greater than value B.
        Otherwise, register C is set to 0."""
        self.registers[code.c] = 1 if self.registers[code.a] > code.b else 0
        self.debug(Frame(self.registers, code, "R", ">", "V"))

    def gtrr(self, code: Code) -> None:
        """(greater-than register/register) sets register C to 1 if register A is greater than register B.
        Otherwise, register C is set to 0."""
        self.registers[code.c] = (
            1 if self.registers[code.a] > self.registers[code.b] else 0
        )
        self.debug(
            Frame(self.registers, code, "R", ">", "R", message="If R3 > R2 keep going; set R3 to '0' else '1'")
        )

    def eqrr(self, code: Code) -> None:
        """(equal register/register) sets register C to 1 if register A is equal to register B.
        Otherwise, register C is set to 0.
        """
        self.registers[code.c] = (
            1 if self.registers[code.a] == self.registers[code.b] else 0
        )
        self.debug(
            Frame(
                self.registers, code, "R", "==", "R", message="IF R0 == R1 then R4 will be '0' else '1'"
            )
        )

    def eqri(self, code: Code) -> None:
        """(equal register/immediate) sets register C to 1 if register A is equal to value B.
        Otherwise, register C is set to 0.
        """
        self.registers[code.c] = 1 if self.registers[code.a] == code.b else 0
        self.debug(Frame(self.registers, code, "R", "==", "V"))


def parse(path: Path) -> Tuple[int, List[Code]]:
    lines = iter(helpers.lines(path))
    ip = int(next(lines).split()[1])
    codes = []
    for line in lines:
        cmd, a, b, c = line.split()
        codes.append(Code(cmd, int(a), int(b), int(c)))

    return ip, codes


def part01() -> int:
    # using the debug shows that line 28 of the program does a comparison of R1, 10332277, to R0.
    # Priming the r0 with this number would create the halting situation.
    test = 10332277
    path = Path(__file__).parent / ".." / "data" / "day_21.txt"
    ip, codes = parse(path)
    for x in range(test, test + 1):
        machine = Machine(ip, codes)
        Machine.DEBUG = False
        machine.registers[0] = x
        r = machine.run()
        return r


def run() -> None:
    assert part01() == 10332277


if __name__ == "__main__":
    run()