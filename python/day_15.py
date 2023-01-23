from __future__ import annotations

from collections import defaultdict
from typing import Tuple, List, Set, Iterator, Dict, Optional

import helpers
from pathlib import Path

Pos = Tuple[int, int]


class Node:
    def __init__(self, state, parent: Optional[Node], depth: int) -> None:
        self.state = state
        self.parent = parent
        self.depth = depth


class Unit:
    def __init__(self, pos: Pos, unit_type: str) -> None:
        self.pos = pos
        self.type = unit_type
        self.hp = 200
        self.ap = 3

    def __lt__(self, other: Unit) -> bool:
        return self.pos < other.pos

    def __str__(self) -> str:
        return f'Unit({self.pos}, {self.type}, {self.hp}'


def _neighbours(pos: Pos, blockers: Set[Pos]) -> Iterator[Pos]:
    # print("\t\tOriginal Pos:", pos)
    for n in read_positions(pos):
        # print("\t\t\tTrying:", n, "Is Blocked:", n in blockers)
        if n not in blockers:
            yield n
    # yield pos


def bfs(start: Pos, goal: Pos, units, walls) -> Optional[Node]:
    blockers = {u.pos for u in units if u.hp > 0 and u.pos != goal}.union(walls)
    q: List[Node] = [Node(start, None, 0)]
    visited: Set[Pos] = {start, }
    while q:
        node = q.pop(0)
        if node.state == goal:
            return node

        for neighbour in _neighbours(node.state, blockers):
            if neighbour not in visited:
                visited.add(neighbour)
                q.append(Node(neighbour, node, node.depth + 1))
    return None


def parse(path: Path) -> Tuple[Set[Pos], Set[Unit], List[List[str]]]:
    board: List[List[str]] = [list(line) for line in helpers.lines(path)]
    walls: Set[Pos] = set()
    units: Set[Unit] = set()
    for y in range(len(board)):
        for x in range(len(board[0])):
            char = board[y][x]
            if char in 'GE':
                units.add(Unit((y, x), char))
                board[y][x] = '.'
            elif char in '#':
                walls.add((y, x))

    return walls, units, board


def pprint(round: int, board: List[List[str]], units: Set[Unit]) -> None:
    """Print out a debug board to review each round's results."""
    print(f"Round: {round:02}: Score: {sum(u.hp for u in units if u.hp>0 and u.type == 'G')}")
    for y in range(len(board)):
        scores: List[str] = []
        for x in range(len(board[0])):
            is_unit = False
            for u in units:
                if u.hp > 0 and u.pos == (y, x):
                    is_unit = True
                    print(u.type, end='')
                    scores.append(f'{u.type}({u.hp})')
            if not is_unit:
                print(board[y][x], end='')

        print(' ', ', '.join(scores))


def read_positions(pos: Pos) -> List[Pos]:
    """Read order is like reading an english textbook from top left to bottom right.  There are
    only four possible positions that can surround an Unit and is guaranteed to be in the sorted
    position of first position, then second.  Or Up, left, right, down."""
    u = pos[0] - 1, pos[1]
    d = pos[0] + 1, pos[1]
    l = pos[0], pos[1] - 1
    r = pos[0], pos[1] + 1

    return [u, l, r, d]


def is_beside(unit: Unit, enemies: List[Unit]) -> bool:
    for e in enemies:
        if e.hp > 0 and e.pos in read_positions(unit.pos):
            return True
    return False


def attack(unit: Unit, enemies: List[Unit]) -> None:
    targets = [e for e in enemies if e.hp > 0 and e.pos in read_positions(unit.pos)]
    assert len(targets) <= 4, "Impossible to have more than four enemy positions surrounding a Unit."

    cheapest_unit = None
    cheapest_cost = 201
    for t in targets:
        if t.hp < cheapest_cost:
            cheapest_cost = t.hp
            cheapest_unit = t

    if cheapest_unit is not None:
        cheapest_unit.hp -= unit.ap


def part01(walls, units, board) -> Optional[int]:
    for round in range(1_000):
        # pprint(round, board, units)
        units = {u for u in units if u.hp > 0}

        for unit in sorted(units):
            # Skip over units that are 'dead': hp of zero or less
            if unit.hp <= 0:
                continue
            elves = sorted([u for u in units if u.type == 'E' and u.hp > 0])
            goblins = sorted([u for u in units if u.type == 'G' and u.hp > 0])
            enemies = elves if unit.type == 'G' else goblins

            # End if no enemies to fight
            if len(enemies) == 0:
                # pprint(round, board, units)
                s = sum(u.hp for u in units if u.hp > 0)
                # print('Found on round: ', round, 'with a sum of', s, '=', round * s, 'for', unit)
                return round * s

            # if one step away from any enemy find the enemy that has the lowest HP in reading order and attack
            if not is_beside(unit, enemies):
                # Else, find the shortest path to all enemies and make a move...
                possibles: Dict[int, List[Tuple[Pos, Pos]]] = defaultdict(list)
                blockers = {u.pos for u in units if u.hp > 0}.union(walls)
                start_steps = list(_neighbours(unit.pos, blockers))

                for enemy in enemies:
                    for neighbour in _neighbours(enemy.pos, blockers):
                        for start_step in start_steps:
                            r = bfs(start_step, neighbour, units, walls)
                            if r is not None:
                                #         TargetD          Enemy sq.   Unit.sq
                                possibles[r.depth].append((enemy.pos, start_step))

                if possibles:
                    # Take te lowest key's enemy, unit
                    k, v = sorted(possibles.items())[0]

                    move = sorted(v, key=lambda x: x[0])[0][1]
                    if move not in blockers:
                        unit.pos = move

            attack(unit, enemies)

    return None


def run() -> None:
    path = Path(__file__).parent / '..' / 'data' / 'day_15.txt'
    walls, units, board = parse(path)

    p1 = part01(walls, units, board)
    assert p1 == 227290


if __name__ == "__main__":
    run()
