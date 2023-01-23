import os
import platform
import time
from types import FunctionType
from typing import List


def time_it(command: FunctionType) -> None:
    t1 = time.perf_counter()
    command()
    print(
        f"[{str(command.__module__)}.{command.__name__}]: Completed in {(time.perf_counter() - t1) * 1_000:0.1f} ms"
    )


def time_it_all(args: List) -> None:
    for arg in args:
        time_it(arg)


def display_ascii(grid: List[List[str]], refresh_rate: float = 0.005) -> None:
    """To be used in a terminal on a Mac or a command line on Windows."""
    os_name = platform.system()

    # Check the name of the operating system
    if os_name == "Windows":
        os.system("cls")
    elif os_name == "Darwin":
        print("\033c", end="")

    for row in grid:
        print("".join(row))

    time.sleep(refresh_rate)
