import re
from pathlib import Path
from typing import List

INT_PATTERN = re.compile(r"[-\d]+")


def lines(path: Path) -> List[str]:
    with open(path, "r", encoding="UTF-8") as text:
        return [line.strip() for line in text.readlines()]


def lines_no_strip(path: Path) -> List[str]:
    with open(path, "r", encoding="UTF-8") as text:
        return [line for line in text.readlines()]


def ints(path: Path) -> List[int]:
    with open(path, "r", encoding="UTF-8") as text:
        return [int(i) for i in text.readlines()]
