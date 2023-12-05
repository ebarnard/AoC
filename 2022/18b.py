import numpy as np

EMPTY = 0
LAVA = 1
STEAM = 2


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

    world = np.full((max_x + 3, max_y + 3, max_z + 3), EMPTY)
    for x, y, z in droplets:
        # Add one so we don't touch edges.
        world[x + 1, y + 1, z + 1] = LAVA

    faces = [0]

    def test(new_wavefront, x, y, z):
        if (
            x < 0
            or y < 0
            or z < 0
            or x >= world.shape[0]
            or y >= world.shape[1]
            or z >= world.shape[2]
        ):
            return

        if world[x, y, z] == LAVA:
            faces[0] += 1
        elif world[x, y, z] == EMPTY:
            new_wavefront.add((x, y, z))

    wavefront = set([(0, 0, 0)])
    while wavefront:
        new_wavefront = set()
        for x, y, z in wavefront:
            world[x, y, z] = STEAM

            test(new_wavefront, x + 1, y, z)
            test(new_wavefront, x - 1, y, z)

            test(new_wavefront, x, y + 1, z)
            test(new_wavefront, x, y - 1, z)

            test(new_wavefront, x, y, z + 1)
            test(new_wavefront, x, y, z - 1)

        wavefront = new_wavefront

    return faces[0]


if __name__ == "__main__":
    assert run(parse("18-test.txt")) == 58
    assert run(parse("18-real.txt")) == 2452
