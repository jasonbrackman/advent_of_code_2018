class Cart:
    compass = {
        "^": [-1, 0],
        "v": [1, 0],
        "<": [0, -1],
        ">": [0, 1],
    }

    def __init__(self, pos, dir):
        self.pos = pos
        self.dir = dir
        self.intersection_count = 0

    def next(self):
        dir = Cart.compass[self.dir]
        self.pos = [self.pos[0] + dir[0], self.pos[1] + dir[1]]

    def next_facing(self, p2):
        if p2 == "\\":
            if self.dir == "^":
                self.dir = "<"
            elif self.dir == "v":
                self.dir = ">"
            elif self.dir == "<":
                self.dir = "^"
            elif self.dir == ">":
                self.dir = "v"
        elif p2 == "/":
            if self.dir == "^":
                self.dir = ">"
            elif self.dir == "v":
                self.dir = "<"
            elif self.dir == "<":
                self.dir = "v"
            elif self.dir == ">":
                self.dir = "^"

        elif p2 == "+":
            index = [-1, 0, 1][self.intersection_count % 3]

            options = [">", "v", "<", "^"]
            index_current = options.index(self.dir)

            self.dir = self.dir if index == 0 else options[(index_current + index) % 4]

            self.intersection_count += 1

    def __str__(self):
        return f"Cart at {self.pos} and facing {self.dir}."


class Track:
    def __init__(self, path):
        self.path = path
        self.track = None
        self.rows = None

        self.carts = []
        self.get_track()
        self.get_carts()

    def get_track(self):
        with open(self.path) as f:
            self.track = [list(l.rstrip()) for l in f]
            self.rows = len(self.track)

    def get_carts(self):
        for i in range(self.rows):
            for j in range(len(self.track[i])):
                c = self.track[i][j]
                if c in "<>^v":
                    hidden = self.get_hidden_track(i, j)
                    assert hidden is not None
                    self.track[i][j] = hidden
                    self.carts.append(Cart([i, j], c))

    def sort_carts(self):
        self.carts = sorted(self.carts, key=lambda c: c.pos)

    def get_hidden_track(self, row, col):
        updates = [-1, 1]

        rows = []
        cols = []
        for update in updates:
            r = row + update
            if r < self.rows and col < len(self.track[r]):
                target = self.track[r][col]
                if target != " ":
                    rows.append(target)

            c = col + update
            if c < len(self.track[row]):
                target = self.track[row][c]
                if target != " ":
                    cols.append(target)

        if len(rows) == 2 and len(cols) == 0:
            return "|"

        if len(rows) == 0 and len(cols) == 2:
            return "-"

        if len(rows) == 2 and len(cols) == 2:

            if (rows[0] == rows[1] == "|") and cols[0] == cols[1] == "+":
                return "+"

            if "+" in cols and "-" in cols or ["+", "+"] == cols or cols == ["-", "-"]:
                return "-"

            if "+" in rows and "|" in rows or ["+", "+"] == rows or ["|", "|"] == rows:
                return "|"

        if len(rows) == 1:

            if len(cols) == 2:
                return "-"

        if len(cols) == 1:
            if len(rows) == 2:
                return "|"

        print(rows, cols)
        raise AssertionError("Should not have reached here.")

    def display(self):
        print()
        for i, r in enumerate(self.track):
            for ii, c in enumerate(r):
                assert c is not None
                found = False
                for cart in self.carts:
                    if not found and cart.pos == [i, ii]:
                        print(cart.dir, end="")
                        found = True
                if not found:
                    print(c, end="")
            print()


def validate_next_move(cart, track):
    if cart.dir in ["<", ">"]:
        if track in ["/", "\\", "+", "-"]:
            return True
    if cart.dir in ["^", "v"]:
        if track in ["/", "\\", "+", "|"]:
            return True
    return False


def main(path):

    crashed_order = []
    tick = 0
    track = Track(path)

    while len(track.carts) > 1:
        track.sort_carts()
        # track.display()
        # input()
        to_remove = []
        for cart in track.carts:
            cart_dir = Cart.compass[cart.dir]
            next_row, next_col = cart.pos[0] + cart_dir[0], cart.pos[1] + cart_dir[1]

            if all([next_row, next_col] != c.pos for c in track.carts):
                try:
                    next_move = track.track[next_row][next_col]
                    if validate_next_move(cart, next_move):
                        cart.next()
                        cart.next_facing(next_move)
                    else:
                        raise ValueError(
                            f"Something doesn't make sense: [{cart.dir}] -> [{next_move}] at cart.pos {cart.pos} at tick {tick}"
                        )
                except IndexError as e:
                    raise IndexError(f"{e}: Fell off the track!")

            else:
                # track.display()
                cart.next()

                for cc in track.carts:
                    if cc.pos not in crashed_order:
                        if cc != cart and cc.pos == cart.pos:
                            crashed_order.append(cart.pos)
                            to_remove.append(cart)
                            to_remove.append(cc)
                            # break

        for remove in to_remove:
            track.carts.remove(remove)
        tick += 1
    last_pos = track.carts[0].pos if track.carts else None
    return crashed_order, last_pos


if __name__ == "__main__":
    path = r".././data/day_13.txt"

    # Return values are stored as Row, Col, but the puzzle output values are col, row
    p1, p2 = main(path)
    assert f"{p1[0][1]}, {p1[0][0]}" == "48, 20", f"received {p1[0]}"
    assert f"{p2[1]}, {p2[0]}" == "59, 64"
