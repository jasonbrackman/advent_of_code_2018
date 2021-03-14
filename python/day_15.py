"""You scan the area, generating a map of the:
 - walls (#),
 - open cavern (.),
 and starting position of every Goblin (G) and Elf (E)
"""

from typing import List, Optional, Tuple, Dict
from collections import deque
from collections import defaultdict
from enum import Enum

import display

POSITIONS = [(-1, 0),  # up
             (0, -1),  # left
             (0, 1),   # right
             (1, 0)]  # down


class UnitEnum(Enum):
    ELF = "E"
    GOBLIN = "G"


class Node:
    def __init__(self, state, previous=None, level=0):
        self.state = state
        self.previous = previous
        self.level = level


class Unit:
    def __init__(self, pos):
        self.pos = pos
        self.attack_power = 3
        self.hit_points = 200

    def is_alive(self):
        return self.hit_points > 0

    def attack(self, target):
        target.hit_points -= self.attack_power

    def __str__(self):
        return f"{type(self).__name__}(pos={self.pos}, hp={self.hit_points}, ap={self.attack_power})"


class Goblin(Unit):
    name = UnitEnum.GOBLIN


class Elf(Unit):
    name = UnitEnum.ELF


class Units:
    def __init__(self):
        self._units: Dict[UnitEnum, List[Unit]] = {
            UnitEnum.ELF: list(),
            UnitEnum.GOBLIN: list(),
        }

    def get(self):
        return (u for u in self._units[UnitEnum.ELF] + self._units[UnitEnum.GOBLIN] if u.is_alive)

    def get_enemies(self, unit):
        key = UnitEnum.ELF if unit.name != UnitEnum.ELF else UnitEnum.GOBLIN
        return (u for u in self._units[key] if u.is_alive)

    def append(self, unit):
        self._units[unit.name].append(unit)


class Game:
    def __init__(self, path):
        self.board = []
        self.units = []
        # self.units2 = Units()
        self.parse(path)

    def parse(self, path):
        with open(path) as handle:
            for rowc, line in enumerate(handle):
                row = []
                for col, c in enumerate(line.strip()):
                    if c == "G":
                        self.units.append(Goblin((rowc, col)))
                        row.append(".")
                    elif c == "E":
                        self.units.append(Elf((rowc, col)))
                        row.append(".")
                    else:
                        row.append(c)
                self.board.append(row)

    def display(self, index=None, silent=False):
        result = []
        for r in range(len(self.board)):
            sub_r = []
            inf_r = [" "]
            for c in range(len(self.board[0])):
                p = [u for u in self.units if u.pos == (r, c)]
                if p:
                    sub_r.append(str(p[0].name.value))
                    inf_r.append(f" {p[0].name.value}({p[0].hit_points})")
                else:
                    sub_r.append(self.board[r][c])
            result.append(sub_r + inf_r)

        if index:
            display.generic_out(
                result, {".": "white", "G": "green", "E": "red", "#": "black"}, "day_15", index
            )

        if not silent:
            for r in result:
                print(''.join(r))

        return result

    def get_targets(self, my_type: type(Unit)) -> List[type(Unit)]:
        return [enemy for enemy in self.units if enemy.is_alive() and type(enemy) != my_type]

    def in_range(self, enemies: List[Unit]) -> set:
        results = set()
        for enemy in enemies:
            if enemy.is_alive():
                results |= set(self.get_neighbours(enemy.pos))
        return results

    def get_shortest_path(self, unit: Unit, target: Tuple[int, int]) -> Optional[Node]:
        visited = set()
        queue = [Node(unit.pos, None)]
        visited.add(unit.pos)

        while queue:
            queue = sorted(queue, key=lambda x: (x.level, x.state))
            node = queue.pop(0)

            if node.state == target:
                return node

            for neighbour in self.get_neighbours(node.state):
                if neighbour in visited:
                    continue
                visited.add(neighbour)
                queue.append(Node(neighbour, node, node.level+1))

        return None

    def reachable(self, unit, target) -> bool:
        visited = set()
        queue = deque(self.get_neighbours(unit.pos))
        visited.add(unit.pos)
        while queue:
            test = queue.popleft()
            if test == target:
                return True

            for n in self.get_neighbours(test):
                if n in visited:
                    continue
                visited.add(n)
                queue.append(n)

        return False

    def nearest(self, unit, positions: List) -> List:
        values_new = defaultdict(list)
        starts = self.get_neighbours(unit.pos)
        for position in positions:
            shortest_paths = [self.get_shortest_path(Unit(start), position) for start in starts]
            shortest_paths = sorted((sp for sp in shortest_paths if sp is not None), key=lambda x: x.level)
            values_new[shortest_paths[0].level].append(position)
        # return min values in read order
        return sorted(values_new[min(values_new)]) if values_new else []

    def sort_units(self):
        """Sort units by reading order.  Left to right and top down."""
        self.units = sorted(self.units, key=lambda k: k.pos)

    @staticmethod
    def get_adjacent_enemies(unit, enemies: List[Unit]) -> List[Unit]:
        r1, c1 = unit.pos
        adjacent_positions = [(r1+r2, c1+c2) for (r2, c2) in POSITIONS]
        return [t for t in enemies if t.pos in adjacent_positions]

    @staticmethod
    def sort_enemies_to_attack(enemies):
        """Enemies are already in Read Order, but those enemies with the lowest HP should
        be returned and sorted."""

        health = defaultdict(list)
        for enemy in enemies:
            health[enemy.hit_points].append(enemy)

        return health[min(health)]

    def get_neighbours(self, pos):
        r1, c1 = pos
        return [(r1 + r2, c1 + c2) for (r2, c2) in POSITIONS if self.is_open(r1+r2, c1+c2)]

    def is_open(self, row, col):
        for p in self.units:
            if p.pos == (row, col):
                return False
        if row < len(self.board) and col < len(self.board[row]):
            return self.board[row][col] == '.'
        return False


def attack_enemies(unit, targets):
    enemies = Game.get_adjacent_enemies(unit, targets)

    if not enemies:
        return False

    sorted_enemies = Game.sort_enemies_to_attack(enemies)
    unit.attack(sorted_enemies[0])

    return True


def round_(g):
    # This is the reading order of their starting positions
    for unit in g.units:
        if not unit.is_alive():
            continue

        # attempt to move in range (if not already in range) and attack (if beside enemy)

        # Identify all possible targets
        targets = g.get_targets(type(unit))
        if not targets:
            return False

        did_attack = attack_enemies(unit, targets)
        if did_attack is False:
            # get the spaces that are available beside units in range
            reachable = [item for item in g.in_range(targets) if g.reachable(unit, item) and item]
            if reachable:
                nearest_chosen = sorted(g.nearest(unit, reachable))[0]

                starts = g.get_neighbours(unit.pos)
                shortest_paths = [g.get_shortest_path(Unit(start), nearest_chosen) for start in starts]
                shortest_paths = sorted((sp for sp in shortest_paths if sp is not None), key=lambda x: x.level)
                unit.pos = get_first_pos(shortest_paths[0])
                attack_enemies(unit, targets)

    return True


def get_first_pos(shortest_path):
    r = shortest_path.state
    while shortest_path.previous:
        shortest_path = shortest_path.previous
        r = shortest_path.state
    return r


def manhattan_distance(pos1, pos2):
    return abs(pos1[0] - pos2[0]) + abs(pos1[1] - pos2[1])


def main():

    paths = [
        # (r".././data/day_15_f0.txt", 27730),
        # (r".././data/day_15_f1.txt", 36334),
        # (r".././data/day_15_f2.txt", 39514),
        # (r".././data/day_15_f3.txt", 27755),
        # (r".././data/day_15_f5.txt", 28944),
        # (r".././data/day_15_f6.txt", 146),
        # (r".././data/day_15_f4.txt", 18740),
        (r".././data/day_15.txt", None)  # fails to find the correct answer
                                         # These failed: 220252,
                                         #               230690,
                                         #               230945,
                                         #               231200,
                                         #               236632,

        # (r".././data/day_15_sp.txt", None)
    ]

    for path, expected in paths:
        g = Game(path)
        index = 0
        should_continue = True
        while should_continue:
            g.display()
            print(f"Round: {index}")
            index += 1

            g.sort_units()
            should_continue = round_(g)
            g.units = [u for u in g.units if u.is_alive()]

        g.units = [u for u in g.units if u.is_alive()]
        g.display()
        total = sum(unit.hit_points for unit in g.units if unit.is_alive()) * (index-1)
        print(path, total)
        assert total == expected, f"Got {total}, expected {expected}"


def generate_gif():
    imgs = display.load_images_starting_with("day_15_")
    imgs[0].save(
        r"./images/day_15.gif",
        save_all=True,
        append_images=imgs[1:],
        duration=1,
        loop=0,
    )


if __name__ == "__main__":
    # import cProfile
    # cProfile.run("main()")
    main()
