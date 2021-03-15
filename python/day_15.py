"""You scan the area, generating a map of the:
 - walls (#),
 - open cavern (.),
 and starting position of every Goblin (G) and Elf (E)
"""
from dataclasses import dataclass
from typing import List, Tuple
from collections import deque
from collections import defaultdict
from enum import Enum

import display

POSITIONS = [(-1, 0), (0, -1), (0, 1), (1, 0)]  # up  # left  # right  # down


class UnitEnum(Enum):
    ELF = "E"
    GOBLIN = "G"


@dataclass
class Unit:
    name: UnitEnum
    pos: Tuple[int, int]
    attack_power = 3
    hit_points = 200

    def neighbours(self):
        return [(self.pos[0] + n[0], self.pos[1] + n[1]) for n in POSITIONS]

    def alive(self):
        return self.hit_points > 0

    def attack(self, target):
        target.hit_points -= self.attack_power

    def __str__(self):
        return f"{self.name.value}(pos={self.pos}, hp={self.hit_points}, ap={self.attack_power})"


class Game(list):
    def __init__(self, path):
        super().__init__()
        self.units = []
        self.parse(path)

    def parse(self, path):
        with open(path) as handle:
            for rowc, line in enumerate(handle):
                row = []
                for col, c in enumerate(line.strip()):
                    if c in "GE":
                        self.units.append(
                            Unit(
                                name={"E": UnitEnum.ELF, "G": UnitEnum.GOBLIN}[c],
                                pos=(rowc, col),
                            )
                        )
                        row.append(".")
                    else:
                        row.append(c)
                self.append(row)

    def display(self, index=None, silent=False):
        result = []
        for r in range(len(self)):
            sub_r = []
            inf_r = [" "]
            for c in range(len(self[0])):
                p = [u for u in self.units if u.pos == (r, c)]
                if p:
                    sub_r.append(str(p[0].name.value))
                    inf_r.append(f" {p[0].name.value}({p[0].hit_points})")
                else:
                    sub_r.append(self[r][c])
            result.append(sub_r + inf_r)

        if index:
            display.generic_out(
                result,
                {".": "white", "G": "green", "E": "red", "#": "black"},
                "day_15",
                index,
            )

        if not silent:
            for r in result:
                print("".join(r))

        return result

    def get_targets(self, unit: Unit) -> List[Unit]:
        return [
            enemy for enemy in self.units if enemy.alive() and enemy.name != unit.name
        ]

    def in_range(self, enemies: List[Unit]) -> set:
        results = set()
        for enemy in enemies:
            if enemy.alive():
                results |= set(
                    [n for n in enemy.neighbours() if self.is_open(n[0], n[1])]
                )
        return results

    def get_shortest_path(self, pos: Tuple[int, int], targets: List[Tuple[int, int]]):
        visited = {
            pos,
        }
        lookup = {pos: [0, None]}
        queue = [(pos, 0)]

        while queue:
            queue = sorted(queue, key=lambda x: (x[1], x[0]))
            state, level = queue.pop(0)

            for neighbour in self.get_neighbours(state):
                if neighbour not in lookup or lookup[neighbour] > [level + 1, state]:
                    lookup[neighbour] = [level + 1, state]
                if neighbour in visited:
                    continue

                visited.add(neighbour)
                queue.append((neighbour, level + 1))

        _, closest = min(
            [distance, p] for p, (distance, goal) in lookup.items() if p in targets
        )
        while lookup[closest][0] > 1:
            closest = lookup[closest][1]
        return closest

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

    def sort_units(self):
        """Sort units by reading order.  Left to right and top down."""
        self.units = sorted(self.units, key=lambda k: k.pos)

    @staticmethod
    def get_adjacent_enemies(unit, enemies: List[Unit]) -> List[Unit]:
        r1, c1 = unit.pos
        adjacent_positions = [(r1 + r2, c1 + c2) for (r2, c2) in POSITIONS]
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
        return [
            (r1 + r2, c1 + c2)
            for (r2, c2) in POSITIONS
            if self.is_open(r1 + r2, c1 + c2)
        ]

    def is_open(self, row, col):
        for p in self.units:
            if p.pos == (row, col):
                return False
        return self[row][col] == "."


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
        if not unit.alive():
            continue

        # attempt to move in range (if not already in range) and attack (if beside enemy)
        targets = g.get_targets(unit)
        if not targets:
            return False

        did_attack = attack_enemies(unit, targets)
        if did_attack is False:
            # get the spaces that are available beside units in range
            reachable = [
                item for item in g.in_range(targets) if g.reachable(unit, item) and item
            ]
            if reachable:
                unit.pos = g.get_shortest_path(unit.pos, reachable)
                attack_enemies(unit, targets)

    return True


def manhattan_distance(pos1, pos2):
    return abs(pos1[0] - pos2[0]) + abs(pos1[1] - pos2[1])


def main():

    paths = [
        (r".././data/day_15_f0.txt", 27730),
        (r".././data/day_15_f1.txt", 36334),
        (r".././data/day_15_f2.txt", 39514),
        (r".././data/day_15_f3.txt", 27755),
        (r".././data/day_15_f5.txt", 28944),
        (r".././data/day_15_f6.txt", 146),
        (r".././data/day_15_f4.txt", 18740),
        (r".././data/day_15.txt", None),  # fails to find the correct answer
        # These failed: 220252,
        #               230690,
        #               230945,
        #               231200,
        #               236632,
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
            g.units = [u for u in g.units if u.alive()]

        g.units = [u for u in g.units if u.alive()]
        g.display()
        total = sum(unit.hit_points for unit in g.units if unit.alive()) * (index - 1)
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
