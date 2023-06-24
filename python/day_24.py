"""
Units within a group all have the same hit points
(amount of damage a unit can take before it is destroyed),
attack damage (the amount of damage each unit deals), an
attack type, an initiative (higher initiative units attack
first and win ties), and sometimes weaknesses or immunities.
"""
from __future__ import annotations

import re
from dataclasses import dataclass
from pathlib import Path
from typing import List, Set, Dict

from python import helpers

PATTERN = re.compile(
    r"(\d+) units each with (\d+) hit points(.*)with an attack.+ (\d+) (\w+) damage.+ (\d+)"
)

Uuid = int
Damage = int


@dataclass
class Unit:
    """Example:
    18 units each with 729 hit points (weak to fire; immune to cold, slashing)
    with an attack that does 8 radiation damage at initiative 10"""

    count: int
    group: int
    uuid: int
    hit_points: int
    attack_damage: int
    attack_type: str
    initiative: int
    weaknesses: Set[str]
    immunities: Set[str]

    def effective_power(self) -> int:
        """effective power: the number of units in that group multiplied by their attack damage.
        Groups never have zero or negative units; instead, the group is removed from combat."""
        return self.count * self.attack_damage

    def receive_damage(self, damage: int) -> None:
        """Calculate and remove units based on the amount of received damage."""

        death = damage // self.hit_points
        # print(
        #     f"\tWith damage of {damage} // {self.hit_points} = {damage // self.hit_points} I am KILLING: {death}"
        # )
        self.count = max(self.count - death, 0)

    def multiplier(self, other: Unit) -> int:
        """Find the multiplier for attacking an enemy based on the immunities and weaknesses."""
        if self.attack_type in other.immunities:
            # enemy is immune
            return 0
        if self.attack_type in other.weaknesses:
            # enemy is weak to attack
            return 2

        return 1


def parse(path: Path, immune_boost: int = 0) -> List[Unit]:
    lines = helpers.lines(path)
    groups: List[Unit] = []
    current = -1
    for index, line in enumerate(lines):
        if not line:
            continue

        items = re.findall(PATTERN, line)
        if not items:
            current += 1
        if items:
            count, hit_points, more, attack, attack_type, initiative = items[0]
            types: Dict[str, List[str]] = {
                "weak": [],
                "immune": [],
            }
            more = more.strip()
            if more:

                results = more.strip().split(";")
                for result in results:
                    result = result.strip().replace("(", "").replace(")", "").split()
                    title = result[0]
                    rest = [r.strip(" ,") for r in result[2:] if r not in " ,"]
                    types[title] = rest

            unit = Unit(
                count=int(count),
                group=current,
                uuid=index,
                attack_damage=int(attack) + immune_boost
                if current == 0
                else int(attack),
                attack_type=attack_type,
                initiative=int(initiative),
                hit_points=int(hit_points),
                weaknesses=set(types["weak"]),
                immunities=set(types["immune"]),
            )

            groups.append(unit)

    return groups


def target_selection(armies: List[Unit]) -> Dict[Uuid, Unit]:
    """
    During the target selection phase, each group attempts to choose one target.

    In decreasing order of effective power, groups choose their targets; in a tie,
    the group with the higher initiative chooses first.

    The attacking group chooses to target the group in the enemy army to which it
    would deal the most damage:
        (after accounting for weaknesses and immunities, but not accounting for
        whether the defending group has enough units to actually receive all of
        that damage).

    If an attacking group is considering two defending groups to which it would deal
    equal damage, it chooses to target the defending group with the largest
    effective power; if there is still a tie, it chooses the defending group with
    the highest initiative. If it cannot deal any defending groups damage, it does
    not choose a target. Defending groups can only be chosen as a target by one
    attacking group.

    At the end of the target selection phase, each group has selected zero or one
    groups to attack, and each group is being attacked by zero or one groups.
    """
    results: Dict[Uuid, Unit] = dict()

    visited: Set[int] = set()

    army = list(
        reversed(sorted(armies, key=lambda x: (x.effective_power(), x.initiative)))
    )
    g0 = [a for a in army if a.group == 0]
    g1 = [a for a in army if a.group == 1]

    for a in army:
        # calculate the three damage options
        dmg_immune = 0
        dmg_normal = a.effective_power()
        dmg_weak = dmg_normal * 2

        # prime the damage collector
        most_damage: Dict[Damage, List[Unit]] = dict()
        most_damage[dmg_immune] = []
        most_damage[dmg_normal] = []
        most_damage[dmg_weak] = []

        # find out which army the current b is in to target the opposite
        r = g1 if a.group == 0 else g0

        # Check if r contains the enemy units....
        for b in r:
            if b.uuid not in visited:
                immune = a.attack_type in b.immunities
                weak = a.attack_type in b.weaknesses

                if immune is True:
                    # skip, ensure it is left untracked
                    continue
                elif weak is True:
                    most_damage[dmg_weak].append(b)
                else:
                    most_damage[dmg_normal].append(b)

        # pull out the best target and note its UUID in visited
        keep_going = True
        for dmg in (dmg_weak, dmg_normal, dmg_immune):
            if keep_going:
                for d in most_damage[dmg]:
                    # print(
                    #     f'{a.uuid}-{"Infection" if a.group == 1 else "Immune"} attacks {"Infection" if d.group == 1 else "Immune"} with {a.attack_type} to generate {dmg} damage against {d.count}.'
                    # )
                    keep_going = False
                    results[a.uuid] = d
                    visited.add(d.uuid)
                    break

    return results


def attacking(armies: List[Unit], attacks: Dict[Uuid, Unit]) -> None:
    """
    During the attacking phase, each group deals damage to the target it selected,
    if any. Groups attack in decreasing order of initiative, regardless of whether
    they are part of the infection or the immune system.

    (If a group contains no units, it cannot attack.)

    """

    army = list(reversed(sorted(armies, key=lambda x: x.initiative)))

    for a in army:
        if a.uuid in attacks:
            b = attacks[a.uuid]
            multiplier = a.multiplier(b)
            b.receive_damage(a.effective_power() * multiplier)
            # print("\tAfter:", b.count)


def _is_two_group(groups: List[Unit]) -> bool:
    group_ids = {g.group for g in groups}
    return len(group_ids) == 2


def part01() -> None:
    """Part01 of the puzzle day."""
    groups = parse(Path(r"../data/day_24.txt"))
    while _is_two_group(groups):
        attacks = target_selection(groups)
        attacking(groups, attacks)
        groups = [g for g in groups if g.count > 0]
    t = sum(g.count for g in groups)
    # print(groups[0].group, "won.")
    assert t == 14000


def part02() -> None:
    # "Infection" == 1
    # "Immune" == 0 <-- we want this one to win
    for x in range(0, 5000):
        groups = parse(Path(r"../data/day_24.txt"), immune_boost=x)

        is_changed = True
        h1 = None
        while is_changed and _is_two_group(groups):
            attacks = target_selection(groups)
            attacking(groups, attacks)
            groups = [g for g in groups if g.count > 0]
            h2 = sum(g.count for g in groups)
            is_changed = h1 != h2
            h1 = h2

        if is_changed:
            t = sum(g.count for g in groups)
            if groups[0].group == 0:
                assert t == 6149
                # print(f'Round [{x}] - ', groups[0].group, "won with a total of", t)
                break


def run() -> None:
    part01()
    part02()


if __name__ == "__main__":
    run()
