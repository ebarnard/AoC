from itertools import chain
import numpy as np

EMPTY = b"."
ROCK = b"#"
SAND = b"o"
SAND_SOURCE = (500, 0)


def parse(input_path):
    input = open(input_path, "r")

    rock_paths = []
    for line in input.readlines():
        line = line.strip()
        coords = line.split(" -> ")
        rock_path = [(int(x), int(y)) for x, y in (c.split(",") for c in coords)]
        rock_paths.append(rock_path)

    return rock_paths


def run(rock_paths):
    # X grows to the left, Y grows down
    min_u = min(min(u for u, _ in chain(*rock_paths)), SAND_SOURCE[0])
    max_u = max(max(u for u, _ in chain(*rock_paths)), SAND_SOURCE[0])
    min_v = min(min(v for _, v in chain(*rock_paths)), SAND_SOURCE[1])
    max_v = max(max(v for _, v in chain(*rock_paths)), SAND_SOURCE[1])

    # Origin is top-left
    height = max_v - min_v + 4
    width = max_u - min_u + 3
    origin = (min_u - 1, min_v - 1)
    sand_source = (SAND_SOURCE[0] - origin[0], SAND_SOURCE[1] - origin[1])
    world = np.full((width, height), EMPTY, dtype="<S1")

    # Add rocks
    for path in rock_paths:
        for (u0, v0), (u1, v1) in zip(path[:-1], path[1:]):
            x0 = u0 - origin[0]
            y0 = v0 - origin[1]
            x1 = u1 - origin[0]
            y1 = v1 - origin[1]

            if x0 == x1:
                for y in range(min(y0, y1), max(y0, y1) + 1):
                    world[x0, y] = ROCK
            elif y0 == y1:
                for x in range(min(x0, x1), max(x0, x1) + 1):
                    world[x, y0] = ROCK
            else:
                assert False

    settled_count = 0
    while True:
        sand_x, sand_y = sand_source
        settled = False
        while not settled and sand_y + 1 < height:
            assert world[sand_x, sand_y] == EMPTY

            # Try to move down
            if world[sand_x, sand_y + 1] == EMPTY:
                sand_y += 1
                continue

            # Try to move down and left
            if world[sand_x - 1, sand_y + 1] == EMPTY:
                sand_x -= 1
                sand_y += 1
                continue

            # Try to move down and right
            if world[sand_x + 1, sand_y + 1] == EMPTY:
                sand_x += 1
                sand_y += 1
                continue

            # Cannot move
            world[sand_x, sand_y] = SAND
            settled = True

        # print_world(world)

        if settled:
            settled_count += 1
        else:
            break
    return settled_count


def print_world(world):
    for row in world.T:
        for cell in row:
            print(cell.decode("ascii"), end="")
        print()


if __name__ == "__main__":
    assert run(parse("14-test.txt")) == 24
    assert run(parse("14-real.txt")) == 692
