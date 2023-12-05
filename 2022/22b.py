from re import A
import types

import numpy as np

cell = types.SimpleNamespace()
cell.OUTSIDE = " "
cell.OPEN = "."
cell.WALL = "#"

direction = types.SimpleNamespace()
direction.RIGHT = 0
direction.DOWN = 1
direction.LEFT = 2
direction.UP = 3

CHANGES_HANDEDNESS = np.array(
    [
        [False, True, True, False],
        [True, False, False, True],
        [True, False, False, True],
        [False, True, True, False],
    ]
)

A = False
B = False
C = False
D = False
E = False
F = False


def parse(input_path):
    input = open(input_path, "r")
    lines = input.read().splitlines()

    world_lines = lines[:-2]
    height = len(world_lines)
    width = max(len(line) for line in world_lines)
    world = np.full((height, width), cell.OUTSIDE)
    for i, line in enumerate(world_lines):
        world[i, : len(line)] = [c for c in line]

    path_text = lines[-1]
    path = []
    distance_text = ""
    for c in path_text:
        if c in "0123456789":
            distance_text += c
        else:
            path.append(int(distance_text))
            distance_text = ""
            path.append(c)
    if distance_text:
        path.append(int(distance_text))

    return world, path


def run(input_path, face_map, edge_map):
    world, path = parse(input_path)

    # Side length
    assert world.shape[0] % face_map.shape[0] == 0
    a = world.shape[0] // face_map.shape[0]
    assert world.shape[1] % a == 0

    # Pad world
    world_padded = np.full(np.array(world.shape) + 1, cell.OUTSIDE, dtype=world.dtype)
    world_padded[0 : world.shape[0], 0 : world.shape[1]] = world
    world = world_padded

    world_face_map = np.kron(face_map, np.full((a, a), 1))

    y = 0
    x = next(x for x in range(world.shape[1]) if world[y, x] == ".")
    dir = direction.RIGHT

    for action in path:
        if isinstance(action, int):
            x, y, dir = move(world, world_face_map, edge_map, a, x, y, dir, action)
        elif isinstance(action, str):
            dir = rotate(dir, action)
        else:
            assert False

    return 1000 * (y + 1) + 4 * (x + 1) + dir


def rotate(dir, action):
    match action:
        case "R":
            return (dir + 1) % 4
        case "L":
            return (dir - 1) % 4


def direction_to_vec(dir):
    match dir:
        case direction.RIGHT:
            return 1, 0
        case direction.DOWN:
            return 0, 1
        case direction.LEFT:
            return -1, 0
        case direction.UP:
            return 0, -1


def flip(dir):
    return rotate(rotate(dir, "R"), "R")


def move(world, world_face_map, edge_map, a, x, y, dir, count):
    global A, B, C, D, E, F

    height, width = world.shape

    while count > 0:
        dx, dy = direction_to_vec(dir)
        u = (x + dx) % width
        v = (y + dy) % height
        ndir = dir

        if world[v, u] == cell.OUTSIDE:
            # Move to new face of cube.
            current_face = world_face_map[y, x]
            new_face, ndir = edge_map[(current_face, dir)]

            # Find distance along cube side for current position.
            y0, x0 = np.argwhere(world_face_map == current_face)[0, :]
            match dir:
                case direction.RIGHT | direction.LEFT:
                    A = True
                    d = y - y0
                case direction.DOWN | direction.UP:
                    B = True
                    d = x - x0

            if CHANGES_HANDEDNESS[dir, ndir]:
                d = a - 1 - d

            v0, u0 = np.argwhere(world_face_map == new_face)[0, :]
            match ndir:
                case direction.RIGHT:
                    C = True
                    u = u0
                    v = v0 + d
                case direction.DOWN:
                    D = True
                    u = u0 + d
                    v = v0
                case direction.LEFT:
                    E = True
                    u = u0 + a - 1
                    v = v0 + d
                case direction.UP:
                    F = True
                    u = u0 + d
                    v = v0 + a - 1

        match world[v, u]:
            case cell.OPEN:
                count -= 1
                x, y, dir = u, v, ndir
            case cell.WALL:
                break
            case _:
                assert False
    return x, y, dir


def with_edge_map_inverse(forward):
    inverse = {(x, flip(y)): (a, flip(b)) for (a, b), (x, y) in forward.items()}
    return {**forward, **inverse}


# Could calculate this by breaking input into squares such that there are exactly 6 non-empty
# square blocks.
TEST_FACE_MAP = np.array([[0, 0, 1, 0], [2, 3, 4, 0], [0, 0, 5, 6]])

# By Euler's formula every convex polyhedron can be converted into a planar graph where the edges
# of the graph are this mapping. Cba to do this though.
TEST_EDGE_MAP = with_edge_map_inverse(
    {
        (1, direction.RIGHT): (6, direction.LEFT),
        (1, direction.LEFT): (3, direction.DOWN),
        (1, direction.UP): (2, direction.DOWN),
        (2, direction.DOWN): (5, direction.UP),
        (2, direction.LEFT): (6, direction.UP),
        (3, direction.DOWN): (5, direction.RIGHT),
        (4, direction.RIGHT): (6, direction.DOWN),
    }
)

REAL_FACE_MAP = np.array(
    [
        [0, 1, 2],
        [0, 3, 0],
        [4, 5, 0],
        [6, 0, 0],
    ]
)

REAL_EDGE_MAP = with_edge_map_inverse(
    {
        (1, direction.LEFT): (4, direction.RIGHT),
        (1, direction.UP): (6, direction.RIGHT),
        (2, direction.RIGHT): (5, direction.LEFT),
        (2, direction.DOWN): (3, direction.LEFT),
        (2, direction.UP): (6, direction.UP),
        (3, direction.LEFT): (4, direction.DOWN),
        (5, direction.DOWN): (6, direction.LEFT),
    }
)

if __name__ == "__main__":
    assert run("22-test.txt", TEST_FACE_MAP, TEST_EDGE_MAP) == 5031
    assert run("22-real.txt", REAL_FACE_MAP, REAL_EDGE_MAP) == 156166
