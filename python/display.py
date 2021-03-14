"""
PPM
    P3
    # The P3 means colors are in ASCII, then 3 columns and 2 rows,
    # then 255 for max color, then RGB triplets
    3 2
    255
    255   0   0
"""
#import helpers

import itertools
import os
import random
import sys
from typing import Dict

from PIL import Image, ImageDraw, ImageFont

BYTE_ORDER = sys.byteorder

HERE = os.path.abspath(os.path.curdir)


class PPM:

    multiplier = 1

    def __init__(self, rows, cols):
        self.COLOURS = {
            "white": "255 255 255\n",
            "black": "0 0 0\n",
            "blackish": "20 20 20\n",
            "green": "105 189 111\n",  # "0 255 0\n",
            "blue": "54 81 94\n",
            "purple": "128 0 128\n",
            "random": f"{random.randint(0, 255)} {random.randint(0, 255)} {random.randint(0, 255)}\n",
            "red": "189 105 105\n",  # "255 0 0\n",
        }

        self.rows = rows * self.multiplier
        self.cols = cols * self.multiplier
        # print(f"Image Size: {self.rows}, {self.cols}")
        # Default background is near black
        self.pixels = [[self.COLOURS["blackish"]] * self.cols for _ in range(self.rows)]

    def pixel(self, row, col, colour: str):
        if colour not in self.COLOURS:
            raise ValueError(
                f"Expected one of the following colours: {self.COLOURS.keys()}"
            )

        r = row * self.multiplier
        c = col * self.multiplier
        if r < self.rows and c < self.cols:
            for incr in range(self.multiplier):
                for incc in range(self.multiplier):
                    self.pixels[r + incr][c + incc] = self.COLOURS[colour]

    def paint(self, file_path: str, fmt: str):
        """

        :param file_path:
        :param fmt: Expecting P3 (ascii) or P6 (binary)
        :return:
        """
        ascii_colours = fmt
        max_colour = 255

        header = f"{ascii_colours} {self.cols} {self.rows} {max_colour} "

        mode = "wt" if fmt == "P3" else "wb+"
        with open(file_path, mode) as handle:
            h = header if fmt == "P3" else header.encode()
            handle.write(h)
            for lines in self.pixels:
                if fmt == "P6":
                    for line in lines:
                        r, g, b = [int(i) for i in line.split()]
                        handle.write(bytes([r, g, b]))
                else:
                    handle.writelines(lines)


def example():
    test_data = [
        "###########",
        "#0.1.....2#",
        "#.#######.#",
        "#4.......3#",
        "###########",
    ]
    rows = len(test_data)
    cols = len(test_data[0])
    for index in range(3):
        canvas = PPM(rows, cols)
        for r in range(rows):
            for c in range(cols):
                colour = "black"
                if test_data[r][c] == ".":
                    colour = "white"
                elif test_data[r][c].isdigit():
                    colour = "random"
                canvas.pixel(r, c, colour)
        canvas.paint(f"./images/test_{index:02}_p3.ppm", fmt="P3")
        canvas.paint(f"./images/test_{index:02}_p6.ppm", fmt="P6")


# def hex_example():
#     start = helpers.HexPos(0, 0, 0)
#     positions = [start]
#     positions += start.neighbours()
#
#     canvas = PPM(100, 100)
#     pixel = (4, 4)
#     for p in positions:
#         print(p.x, p.y, p.z)


def load_images_starting_with(prefix, title=False):
    # root = os.path.join(os.getcwd(), 'images')

    imgs = []
    index = 0
    root = os.path.join(os.getcwd(), "images")
    for f in sorted(os.listdir(root)):
        if f.startswith(prefix) and f.endswith(".ppm"):
            try:
                im = Image.open(os.path.join(root, f))
                if title:
                    draw = ImageDraw.Draw(im)
                    draw.rectangle(((0, 0), (120, 15)), fill=(0, 0, 0))
                    draw.text(
                        (0, 0), f"{f:>10}", font=ImageFont.truetype("Verdana.ttf")
                    )
                imgs.append(im)
                index += 1
            except:
                print(f)
    return imgs


def generic_out(data, rules: Dict[chr, str], prefix, index):
    multiplier = 6
    final = []
    for d in data:
        d = [d] * multiplier
        d = list(itertools.chain.from_iterable(zip(*d)))
        for _ in range(multiplier):
           final.append(d)
    data = final

    rows = len(data)
    cols = len(data[0])

    canvas = PPM(rows, cols)
    for r in range(rows):
        for c in range(cols):
            colour = rules.get(data[r][c], "black")
            canvas.pixel(r, c, colour)

    canvas.paint(f"{HERE}/images/{prefix}_{index:03}.ppm", fmt="P6")


if __name__ == "__main__":
    # example()
    # hex_example()
    pass
