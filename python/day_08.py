from typing import List, Optional


def parse(path):
    with open(path, "r") as f:
        return [int(i) for i in f.read().split()]


class Node:
    def __init__(self, children_count, meta_count, meta=None, parent=None, child=None):
        self.children_count: int = children_count
        self.meta_count: int = meta_count
        self.parent = parent
        self.children = [child] if child else []
        self.meta: Optional[List] = meta


def recurse(root):
    total = 0
    for child in root.children:
        total += sum(child.meta)
        if child.children:
            total += recurse(child)
    return total


def part01(data):
    # get Root Node
    a = data.pop(0)
    b = data.pop(0)
    c = [data.pop() for _ in range(b)]
    current = Node(a, b, c, parent=None)

    while data:
        a = data.pop(0)
        b = data.pop(0)

        new_node = Node(a, b, parent=current)

        current.children.append(new_node)
        current = new_node

        while len(current.children) == current.children_count:
            if not current.meta:
                current.meta = [data.pop(0) for _ in range(current.meta_count)]

            for child in reversed(current.children):
                if not child.meta:
                    child.meta = [data.pop(0) for _ in range(child.meta_count)]

            if current.parent is None:
                break

            current = current.parent

    return current


def part02(root):
    total = 0
    if root.children_count == 0:
        total += sum(root.meta)

    else:
        for i in root.meta:
            try:
                total += part02(root.children[i - 1])
            except IndexError:
                pass

    return total


if __name__ == "__main__":
    path_ = r".././data/day_08.txt"
    data = parse(path_)

    p1 = part01(data)
    total = sum(p1.meta) + recurse(p1)
    assert total == 44838

    p2 = part02(p1)
    assert p2 == 22198
