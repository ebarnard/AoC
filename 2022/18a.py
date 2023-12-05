import numpy as np


def parse(input_path):
    input = open(input_path, "r")
    lines = input.readlines()

    return np.array([parse_line(l) for l in lines])


def parse_line(line):
    x, y, z = line.strip().split(",")
    return [int(x), int(y), int(z)]


def run(droplets):
    max_x = max(droplets[:, 0])
    max_y = max(droplets[:, 1])
    max_z = max(droplets[:, 2])

    world = np.full((max_x + 2, max_y + 2, max_z + 2), False)
    for x, y, z in droplets:
        # Don't need to add one here due to wrap-around.
        world[x, y, z] = True

    faces = [0]

    def test(x, y, z):
        if not world[x, y, z]:
            faces[0] += 1

    for x, y, z in droplets:
        test(x + 1, y, z)
        test(x - 1, y, z)

        test(x, y + 1, z)
        test(x, y - 1, z)

        test(x, y, z + 1)
        test(x, y, z - 1)

    return faces[0]


if __name__ == "__main__":
    assert run(parse("18-test.txt")) == 64
    assert run(parse("18-real.txt")) == 4282
