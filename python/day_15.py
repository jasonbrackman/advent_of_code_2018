"""You scan the area, generating a map of the:
 - walls (#),
 - open cavern (.),
 and starting position of every Goblin (G) and Elf (E)
"""

from typing import List, Tuple
from collections import deque
from collections import defaultdict

POSITIONS = [(-1, 0),  # up
             (0, -1),  # left
             (0, 1),   # right
             (1, 0)]  # down


class Node:
    def __init__(self, state, previous=None, level=0):
        self.state = state
        self.previous = previous


class Unit:
    def __init__(self, pos):
        self.pos = pos
        self.attack_power = 3
        self.hit_points = 200

    def is_alive(self):
        return self.hit_points > 0

    def attack(self, target):
        target.hit_points -= self.attack_power

    def manhattan_distance(self, pos):
        return abs(self.pos[0] - pos[0]) + abs(self.pos[1] - pos[1])

    def __str__(self):
        return f"{type(self).__name__}(pos={self.pos}, hp={self.hit_points}, ap={self.attack_power})"


class Goblin(Unit):
    name = "G"


class Elf(Unit):
    name = "E"


class Game:
    def __init__(self, path):
        self.board = []
        self.units = []
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

    def display(self, silent=False):
        result = []
        for r in range(len(self.board)):
            sub_r = []
            inf_r = [" "]
            for c in range(len(self.board[0])):
                p = [u for u in self.units if u.pos == (r, c)]
                if p:
                    sub_r.append(str(p[0].name))
                    inf_r.append(f" {p[0].name}({p[0].hit_points})")
                else:
                    sub_r.append(self.board[r][c])
            result.append(sub_r + inf_r)

        if not silent:
            for r in result:
                print(''.join(r))

        return result

    def get_targets(self, my_type: type(Unit)) -> List[type(Unit)]:
        return [enemy for enemy in self.units if enemy.is_alive() and type(enemy) != my_type]

    def in_range(self, enemies: List[Unit]) -> List:
        results = []
        for enemy in enemies:
            results.extend(self.get_neighbours(enemy.pos))
        return results

    def get_shortest_path(self, unit: Unit, target: Tuple[int, int]):
        visited = set()
        queue = deque([Node(unit.pos, None)])
        visited.add(unit.pos)

        while queue:
            queue = deque(sorted(queue, key=lambda x: abs(x.state[0] - target[0]) + abs(x.state[1] - target[1])))
            node = queue.popleft()

            if node.state == target:
                return node

            neighbours = self.get_neighbours(node.state)
            for neighbour in neighbours:
                if neighbour in visited:
                    continue

                visited.add(neighbour)
                queue.append(Node(neighbour, node))

        return None

    def reachable(self, unit, target) -> bool:
        visited = set()
        neighbours = deque(self.get_neighbours(unit.pos))
        visited.add(unit.pos)
        while neighbours:
            test = neighbours.popleft()
            if test == target:
                return True

            for n in self.get_neighbours(test):
                if n in visited:
                    continue
                visited.add(n)
                neighbours.append(n)

        return False

    @staticmethod
    def nearest(unit, positions: List) -> List:
        values = defaultdict(list)
        for position in positions:
            r = unit.manhattan_distance(position)
            values[r].append(position)

        return values[min(values)] if values else []

    def sort_units(self):
        """Sort units by reading order.  Left to right and top down."""
        self.units = sorted(self.units, key=lambda k: k.pos)

    @staticmethod
    def get_adjacent_enemies(unit, enemies):
        results = []
        r1, c1 = unit.pos
        for (r2, c2) in POSITIONS:
            row, col = r1 + r2, c1 + c2
            results.extend([t for t in enemies if t.pos == (row, col)])
        return results

    @staticmethod
    def sort_enemies_to_attack(enemies):
        """Enemies are already in Read Order, but those enemies with the lowest HP should
        be returned and sorted."""

        health = defaultdict(list)
        for enemy in enemies:
            health[enemy.hit_points].append(enemy)

        return health[min(health)]

    def get_neighbours(self, enemy_pos):
        r1, c1 = enemy_pos
        return [(r1 + r2, c1 + c2) for (r2, c2) in POSITIONS if self.is_open(r1+r2, c1+c2)]

    def is_open(self, row, col):
        for p in self.units:
            if p.pos == (row, col):
                return False
        return self.board[row][col] == '.'


def attack_enemies(unit, targets):
    enemies = Game.get_adjacent_enemies(unit, targets)

    if not enemies:
        return False

    sorted_enemies = Game.sort_enemies_to_attack(enemies)
    unit.attack(sorted_enemies[0])

    return True


def round_(g):
    for unit in g.units:
        if not unit.is_alive():
            continue
        targets = g.get_targets(type(unit))
        if not targets:
            return False

        did_attack = attack_enemies(unit, targets)
        if did_attack is False:
            in_range = g.in_range(targets)
            reachable = [item for item in in_range if g.reachable(unit, item)]
            if reachable:
                nearest_chosen = sorted(g.nearest(unit, reachable))[0]
                shortest_path = g.get_shortest_path(unit, nearest_chosen)
                unit.pos = get_next_pos(shortest_path)
                attack_enemies(unit, targets)

    return True


def get_next_pos(shortest_path):
    moves = [shortest_path.state]
    while shortest_path.previous:
        shortest_path = shortest_path.previous
        moves.append(shortest_path.state)
    return moves[-2]


def main():

    paths = [
        (r".././data/day_15_f0.txt", 27730),
        (r".././data/day_15_f1.txt", 36334),
        (r".././data/day_15_f2.txt", 39514),
        (r".././data/day_15_f3.txt", 27755),
        (r".././data/day_15_f5.txt", 28944),
        (r".././data/day_15_f6.txt", 146),
        (r".././data/day_15_f4.txt", 18740),  # error
        # (r".././data/day_15.txt", None)  # fails to find the correct answer
    ]

    for path, expected in paths:
        g = Game(path)
        index = 0
        should_continue = True
        while should_continue:
            g.units = [u for u in g.units if u.is_alive()]
            g.display()
            print(f"Round: {index}")
            g.sort_units()

            should_continue = round_(g)
            index += 1

        g.units = [u for u in g.units if u.is_alive()]
        g.display()
        total = sum(unit.hit_points for unit in g.units if unit.is_alive()) * (index-1)
        print(path, total)
        assert total == expected, f"Got {total}, expected {expected}"


if __name__ == "__main__":
    # import cProfile
    # cProfile.run("main()")
    main()

"bad: 220252, 230945, 236632"