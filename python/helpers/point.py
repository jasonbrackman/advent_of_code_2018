from __future__ import annotations

from math import sqrt


class Pos:
    """Create a 2D position that contains the row and col values."""

    def __init__(self, row: int, col: int):
        self.row = row
        self.col = col

    def __add__(self, other: Pos) -> Pos:
        return Pos(self.row + other.row, self.col + other.col)

    def __sub__(self, other: Pos) -> Pos:
        return Pos(self.row - other.row, self.col - other.col)

    def __eq__(self, other):
        return self.col == other.col and self.row == other.row

    def manhattan_distance(self, other: Pos) -> int:
        y: int = abs(self.row - other.row)
        x: int = abs(self.col - other.col)
        return y + x

    def euclidean_distance(self, other: Pos) -> float:
        y: int = self.row - other.row
        x: int = self.col - other.col
        return sqrt((y * y) + (x * x))

    def __repr__(self):
        return f"Pos({self.row}, {self.col})"
