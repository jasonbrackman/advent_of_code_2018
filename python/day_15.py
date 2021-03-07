"""You scan the area, generating a map of the:
 - walls (#),
 - open cavern (.),
 and starting position of every Goblin (G) and Elf (E)
"""
from typing import List
from collections import deque
from collections import defaultdict

POSITIONS = [
             (-1, 0),  # up
             (0, -1),  # left
             (0, 1),  # right
             (1, 0),  # down
            ]


class Node:
    def __init__(self, state, previous=None):
        self.state = state
        self.previous = previous


class Unit:
    def __init__(self, pos):
        self.pos = pos
        self.attack_power = 3
        self.hit_points = 200

    def is_alive(self):
        return self.hit_points > 0

    def identify_targets(self):
        pass

    def attack(self, target):
        target.hit_points -= self.attack_power

    def move(self, target):
        pass

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

    # @property
    # def units(self):
    #     return self._units
    #     # return [unit for unit in self._units if unit.is_alive()]

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
        return [enemy for enemy in self.units if type(enemy) != my_type]

    def in_range(self, enemies: List) -> List:
        results = []
        for enemy in enemies:
            results.extend(self.get_neighbours(enemy))
        return results

    def get_shortest_path(self, unit, target):
        visited = set()
        queue = deque([Node(unit.pos, None)])
        while queue:
            node = queue.popleft()
            visited.add(node.state)

            if node.state == target:
                return node

            nn = self.get_neighbours(Unit(node.state))
            queue.extend([Node(n, node) for n in nn if n not in visited])

        return None

    def reachable(self, unit, target) -> bool:
        visited = set()
        neighbours = self.get_neighbours(unit)
        while neighbours:
            test = neighbours.pop()
            visited.add(test)

            if test == target:
                return True

            nn = self.get_neighbours(Unit(test))
            neighbours.extend([n for n in nn if n not in visited])

        return False

    @staticmethod
    def nearest(unit, positions: List) -> List:
        values = dict()
        for position in positions:
            r = unit.manhattan_distance(position)
            if r not in values:
                values[r] = [position]
            else:
                values[r].append(position)
        if values:
            return values[min(values)]

        return []

    def chosen(self, enemies: List) -> Unit:
        ...

    def sort_units(self):
        """Sort units by reading order.  Left to right and top down."""
        self.units = sorted(self.units, key=lambda k: k.pos)

    @staticmethod
    def get_adjacent_enemies(unit, enemies):
        results = []
        r1, c1 = unit.pos
        for (r2, c2) in POSITIONS:
            results.extend([t for t in enemies if t.pos == (r1 + r2, c1 + c2)])
        return results

    @staticmethod
    def sort_enemies_to_attack(enemies):
        """Enemies are already in Read Order, but those enmies with the lowest HP should
        be returned and sorted."""
        health = defaultdict(list)
        for enemy in enemies:
            health[enemy.hit_points].append(enemy)
        key = min(health)
        return health[key]

    def get_neighbours(self, enemy):
        results = []

        r1, c1 = enemy.pos
        for (r2, c2) in POSITIONS:
            if self.is_open(r1 + r2, c1 + c2):
                results.append((r1+r2, c1+c2))
        return results

    def is_open(self, row, col):

        if self.board[row][col] == '.':
            for u in self.units:
                if u.pos == (row, col):
                    return False
            return True
        return False


def test_read_order():
    path = r".././data/day_15_test_scan.txt"
    g = Game(path)
    g.sort_units()

    for i, u in enumerate(g.units, 1):
        u.name = i

    expected = [list("#######"),
                list("#.1.2.#"),
                list("#3.4.5#"),
                list("#.6.7.#"),
                list("#######"),
                ]
    result = g.display(silent=True)
#    for expect, result in zip(expected, result):
#        assert ''.join(expect) == ''.join(result), (expect, result)


def attack_enemies(g, unit, targets):
    enemies = Game.get_adjacent_enemies(unit, targets)
    if not enemies:
        return False
    sorted_enemies = Game.sort_enemies_to_attack(enemies)
    assert 0 < len(sorted_enemies) <= 4
    # print(f"{unit} is Attacking: ", sorted_enemies[0])
    unit.attack(sorted_enemies[0])
    if not sorted_enemies[0].is_alive():
        g.units.remove(sorted_enemies[0])
    return True


def round_(g):
    for unit in g.units:

        targets = g.get_targets(type(unit))
        if not targets:
            print("No More Targets!")
            return False
        did_attack = attack_enemies(g, unit, targets)
        if did_attack is False:
            inrange = g.in_range(targets)

            reachable = [item for item in sorted(inrange) if g.reachable(unit, item)]

            if reachable:
                nearest_chosen = sorted(g.nearest(unit, reachable))[0]
                shortest_path = g.get_shortest_path(unit, nearest_chosen)

                moves = [shortest_path.state]
                while shortest_path.previous:
                    shortest_path = shortest_path.previous
                    moves.append(shortest_path.state)

                new_pos = moves[-2]
                unit.pos = new_pos
                attack_enemies(g, unit, targets)
    return True


def main():
    test_read_order()

    path = r".././data/day_15_test_d.txt"
    g = Game(path)
    index = 0
    should_continue = True
    while should_continue:
        g.display()
        print(f"Round: {index}")
        g.sort_units()
        should_continue = round_(g)

        index += 1
        # if index == 2:
        #     should_continue = False
    total = sum(unit.hit_points for unit in g.units)
    print(total * (index-1))


if __name__ == "__main__":
    # import cProfile
    # cProfile.run("main()")
    main()
