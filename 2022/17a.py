from itertools import cycle
import numpy as np

ROCK_COUNT = 2022
ROCK_MAX_HEIGHT = 4
ROCK_MAX_WIDTH = 4

WIDTH = 7
HEIGHT = ROCK_COUNT * ROCK_MAX_HEIGHT

ROCKS = [
    np.array(
        [
            [False, False, False, False],
            [False, False, False, False],
            [False, False, False, False],
            [True, True, True, True],
        ]
    ),
    np.array(
        [
            [False, False, False, False],
            [False, True, False, False],
            [True, True, True, False],
            [False, True, False, False],
        ]
    ),
    np.array(
        [
            [False, False, False, False],
            [False, False, True, False],
            [False, False, True, False],
            [True, True, True, False],
        ]
    ),
    np.array(
        [
            [True, False, False, False],
            [True, False, False, False],
            [True, False, False, False],
            [True, False, False, False],
        ]
    ),
    np.array(
        [
            [False, False, False, False],
            [False, False, False, False],
            [True, True, False, False],
            [True, True, False, False],
        ]
    ),
]


def run(input):
    world = np.full((HEIGHT + 1, WIDTH + ROCK_MAX_WIDTH + 1), False)
    # Add walls
    world[:, 0] = True
    world[:, 1 + WIDTH] = True
    # And floor
    world[0, :] = True

    jets = cycle(input)

    height = 0
    for i in range(ROCK_COUNT):
        # Flip as we are "falling" upwards towards zero.
        rock = np.flip(ROCKS[i % len(ROCKS)], axis=0)
        rock_height = np.sum(np.any(rock, axis=1)).item()

        # Bottom left corner of rock
        # Always two units away from left wall.
        rock_x = 2
        # Three units above the highest point
        rock_y = height + 3

        while True:
            # First, jet of gas pushes rock.
            jet = next(jets)

            if jet == ">":
                move = 1
            elif jet == "<":
                move = -1
            else:
                assert False

            # print_world(world, rock, rock_x, rock_y)

            if not collides(world, rock, rock_x + move, rock_y):
                # No collision. Jet moves rock.
                rock_x += move

            # print_world(world, rock, rock_x, rock_y)

            # Second, rock falls downwards.
            if collides(world, rock, rock_x, rock_y - 1):
                # Collision with world. Rock stops here.
                add(world, rock, rock_x, rock_y)
                height = max(height, rock_y + rock_height)
                break
            else:
                # No collision. Rock continues downwards.
                rock_y -= 1

            pass

        pass

    # print_world(world, rock, rock_x, rock_y)
    return height


def collides(world, rock, rock_x, rock_y):
    rx = slice(1 + rock_x, 1 + rock_x + rock.shape[1])
    ry = slice(1 + rock_y, 1 + rock_y + rock.shape[0])
    region = world[ry, rx]
    return np.any(np.logical_and(rock, region))


def add(world, rock, rock_x, rock_y):
    rx = slice(1 + rock_x, 1 + rock_x + rock.shape[1])
    ry = slice(1 + rock_y, 1 + rock_y + rock.shape[0])
    world[ry, rx] = np.logical_or(world[ry, rx], rock)


def print_world(world, rock, rock_x, rock_y):
    print("World:")
    top = rock_y + ROCK_MAX_HEIGHT + 1
    for y in range(top, max(-1, top - 30), -1):
        for x in range(0, WIDTH + 2):
            if y >= rock_y + 1 and y < rock_y + 1 + rock.shape[0]:
                if x >= rock_x + 1 and x < rock_x + 1 + rock.shape[1]:
                    if rock[y - (rock_y + 1), x - (rock_x + 1)]:
                        print("*", end="")
                        continue

            if world[y, x]:
                print("#", end="")
            else:
                print(".", end="")

        print("")


if __name__ == "__main__":
    assert run(open("17-test.txt").read().strip()) == 3068
    assert run(open("17-real.txt").read().strip()) == 3117
