import types

import numpy as np

cell = types.SimpleNamespace()
cell.EMPTY = 0
cell.ELF = 1

move = types.SimpleNamespace()

STATIC_FILTER = np.array(
    [
        [1, 1, 1],
        [1, 0, 1],
        [1, 1, 1],
    ]
)

NORTH_FILTER = np.array(
    [
        [1, 1, 1],
        [0, 0, 0],
        [0, 0, 0],
    ]
)

SOUTH_FILTER = np.array(
    [
        [0, 0, 0],
        [0, 0, 0],
        [1, 1, 1],
    ]
)

WEST_FILTER = np.array(
    [
        [1, 0, 0],
        [1, 0, 0],
        [1, 0, 0],
    ]
)

EAST_FILTER = np.array(
    [
        [0, 0, 1],
        [0, 0, 1],
        [0, 0, 1],
    ]
)

FILTERS = [
    (NORTH_FILTER, 0, -1),
    (SOUTH_FILTER, 0, 1),
    (WEST_FILTER, -1, 0),
    (EAST_FILTER, 1, 0),
]


def parse(input_path):
    input = open(input_path, "r")
    lines = input.read().splitlines()
    return np.array([[c for c in line] for line in lines])


def run(input_path):
    world = parse(input_path) == "#"

    # Always allow one empty cell around each side of the world.
    world = expand_world(world)
    print("Initial")
    print_world(world)

    for i in range(10):
        move_count = np.zeros_like(world, dtype=np.int32)
        dxs = np.zeros_like(world, dtype=np.int32)
        dys = np.zeros_like(world, dtype=np.int32)

        for y in range(1, world.shape[0] - 1):
            for x in range(1, world.shape[1] - 1):
                if world[y, x] != cell.ELF:
                    continue

                surround = world[y - 1 : y + 2, x - 1 : x + 2]
                if np.sum(surround * STATIC_FILTER) == 0:
                    continue

                for filter_idx in range(i, i + 4):
                    filter, dx, dy = FILTERS[filter_idx % 4]
                    if np.sum(surround * filter) == 0:
                        move_count[y + dy, x + dx] += 1
                        dxs[y, x] = dx
                        dys[y, x] = dy
                        break

        new_world = np.full_like(world, cell.EMPTY)
        for y in range(1, world.shape[0] - 1):
            for x in range(1, world.shape[1] - 1):
                if world[y, x] != cell.ELF:
                    continue

                dx = dxs[y, x]
                dy = dys[y, x]

                if (dx != 0 or dy != 0) and move_count[y + dy, x + dx] == 1:
                    new_world[y + dy, x + dx] = cell.ELF
                else:
                    new_world[y, x] = cell.ELF

        # Always allow one empty cell around each side of the world.
        world = expand_world(new_world)

        print("Round", i + 1)
        print_world(world)

    # Find number of empty ground tiles.
    empty_count = np.sum(world[1:-1, 1:-1] == cell.EMPTY)
    return empty_count


def expand_world(world):
    left = min(1, np.sum(world[:, 0] != cell.EMPTY).item())
    top = min(1, np.sum(world[0, :] != cell.EMPTY).item())
    right = min(1, np.sum(world[:, -1] != cell.EMPTY).item())
    bottom = min(1, np.sum(world[-1, :] != cell.EMPTY).item())
    expanded = np.full(
        (world.shape[0] + top + bottom, world.shape[1] + left + right),
        cell.EMPTY,
        dtype=world.dtype,
    )
    expanded[top : top + world.shape[0], left : left + world.shape[1]] = world
    return expanded


def print_world(world):
    for y in range(0, world.shape[0]):
        for x in range(0, world.shape[1]):
            if world[y, x] == cell.ELF:
                print("#", end="")
            else:
                print(".", end="")
        print()


if __name__ == "__main__":
    assert run("23-test.txt") == 110
    assert run("23-real.txt") == 65368
