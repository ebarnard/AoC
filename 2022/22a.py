import numpy as np

OUTSIDE = " "
OPEN = "."
WALL = "#"


def parse(input_path):
    input = open(input_path, "r")
    lines = input.read().splitlines()

    world_lines = lines[:-2]
    height = len(world_lines)
    width = max(len(line) for line in world_lines)
    world = np.full((height, width), OUTSIDE)
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


def run(input_path):
    world, path = parse(input_path)

    y = 0
    x = next(x for x in range(world.shape[1]) if world[y, x] == ".")
    dir = 0  # Right

    for action in path:
        if isinstance(action, int):
            x, y = move(world, x, y, dir, action)
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


def move(world, x, y, dir, count):
    match dir:
        case 0:
            # Right
            dx, dy = 1, 0
        case 1:
            # Down
            dx, dy = 0, 1
        case 2:
            # Left
            dx, dy = -1, 0
        case 3:
            # Up
            dx, dy = 0, -1

    height, width = world.shape
    nx, ny = x, y
    while count > 0:
        nx = (nx + dx) % width
        ny = (ny + dy) % height
        match world[ny, nx]:
            case " ":
                # Move but don't count as a move.
                pass
            case ".":
                count -= 1
                x, y = nx, ny
            case "#":
                break
            case _:
                assert False
    return x, y


if __name__ == "__main__":
    assert run("22-test.txt") == 6032
    assert run("22-real.txt") == 65368
