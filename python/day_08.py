from typing import List, Optional


def parse(path):
    with open(path, 'r') as f:
        return [int(i) for i in f.read().split()]


class Node:
    def __init__(self, children_count, meta_count, meta=None, parent=None, child=None):
        self.children_count: int = children_count
        self.meta_count: int = meta_count
        self.parent = parent
        self.children = [child] if child else []
        self.meta: Optional[List] = meta

    def __str__(self):
        return f"Children Count: {self.children_count}, Meta Count: {self.meta_count}, Parent: {self.parent}, Children: {self.children}, Meta: {self.meta}"


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
    c = [data.pop() for r in range(b)]
    current = Node(a, b, c, parent=None)
    while data:
        # print(data)
        a = data.pop(0)
        b = data.pop(0)

        new_node = Node(a, b, parent=current)

        current.children.append(new_node)
        current = new_node

        while len(current.children) == current.children_count:
            if not current.meta:
                current.meta = [data.pop(0) for r in range(current.meta_count)]

            for child in reversed(current.children):
                if not child.meta:
                    child.meta = [data.pop(0) for r in range(child.meta_count)]

            if current.parent is not None:
                current = current.parent
            else:
                break
    total = sum(current.meta)
    total += recurse(current)
    return total


if __name__ == "__main__":
    path_ = r'.././data/day_08.txt'
    data = parse(path_)

    p1 = part01(data)
    assert p1 == 44838