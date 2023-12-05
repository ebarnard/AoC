import numpy as np


def run(input_path):
    input = open(input_path, "r")

    heights = []
    for line in input.readlines():
        line = line.strip()
        heights.append([int(h) for h in line])
    heights = np.array(heights)

    is_visible_from_left = visible_from_left(heights)
    is_visible_from_right = visible_from_right(heights)
    is_visible_from_top = visible_from_left(heights.T).T
    is_visible_from_bottom = visible_from_right(heights.T).T

    is_visible_from_any = np.bitwise_or(
        np.bitwise_or(is_visible_from_left, is_visible_from_right),
        np.bitwise_or(is_visible_from_top, is_visible_from_bottom),
    )

    total_visible = np.sum(is_visible_from_any)
    return total_visible


def visible_from_right(heights):
    return np.flip(visible_from_left(np.flip(heights, axis=1)), axis=1)


def visible_from_left(heights):
    is_visible = np.full_like(heights, False)

    for row in range(heights.shape[0]):
        max_height = -1

        for col in range(heights.shape[1]):
            height = heights[row, col]

            if height > max_height:
                is_visible[row, col] = True
                max_height = height

            if max_height == 9:
                break

    return is_visible


if __name__ == "__main__":
    assert run("8-test.txt") == 21
    assert run("8-real.txt") == 1854
