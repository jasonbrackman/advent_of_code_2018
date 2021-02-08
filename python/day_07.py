from collections import defaultdict
from string import ascii_uppercase


def get_steps(path):
    steps = defaultdict(list)
    with open(path) as f:
        for line in f:
            items = line.strip().split()
            steps[items[1]].append(items[7])
    return steps


def get_firsts(steps):
    values = {v for vv in steps.values() for v in vv}
    return [s for s in steps if s not in values]


def bfs(graph, nodes):
    order = []
    q = nodes

    while q:
        q.sort()
        node = q.pop(0)

        while any(node in v for k, v in graph.items() if k not in order):
            q.append(node)
            node = q.pop(0)

        if node not in order:
            order.append(node)
            for cur in graph[node]:
                q.append(cur)

    return order


def bfs2(graph, nodes):
    jobs = []
    order = []
    q = nodes

    while q:
        q.sort()
        node = q.pop(0)

        while any(node in v for k, v in graph.items() if k not in order):
            q.append(node)
            node = q.pop(0)

        if node not in order:
            order.append(node)
            jobs.append(graph[node])
            for cur in graph[node]:
                q.append(cur)

    return order, jobs


class Tick:
    val = {c: i + 60 for i, c in enumerate(ascii_uppercase, 1)}
    tick = 0

    def __init__(self, char):
        self.char = char
        self.current = 0

    def __iter__(self):
        return self

    def __next__(self):
        self.current += 1
        if self.current < self.val[self.char]:
            return None
        elif self.current == self.val[self.char]:
            return self.char
        else:
            raise StopIteration


def part2(job, graph, count=5):
    order = []
    queued = job
    workers = []
    while queued or workers:
        while len(workers) < count and queued:

            node = queued.pop(0)
            if not any(node in v for k, v in graph.items() if k not in order):
                workers.append(Tick(node))

        for i in range(len(workers)):
            worker = workers.pop(0)
            try:
                r = next(worker)
                if r is not None:
                    queued.extend(graph[r])
                    order.append(r)
                else:
                    workers.append(worker)
            except StopIteration:
                pass

        Tick.tick += 1

    return Tick.tick


if __name__ == "__main__":
    path = r".././data/day_07_test.txt"
    steps = get_steps(path)
    firsts = get_firsts(steps)
    p0 = bfs(steps, firsts)
    assert "".join(p0) == "CABDFE", p0

    path = r".././data/day_07.txt"
    steps = get_steps(path)
    first = get_firsts(steps)
    p1 = bfs(steps, first)
    assert "".join(p1) == "ABLCFNSXZPRHVEGUYKDIMQTWJO"

    steps = get_steps(path)
    first = get_firsts(steps)
    p2 = part2(first, steps)
    assert p2 == 1157
