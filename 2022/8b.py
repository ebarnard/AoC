import numpy as np


def run(input_path):
    input = open(input_path, "r")

    heights = []
    for line in input.readlines():
        line = line.strip()
        heights.append([int(h) for h in line])
    heights = np.array(heights)

    max_score = 0
    for row in range(heights.shape[0]):
        for col in range(heights.shape[1]):
            max_score = max(max_score, scenic_score(heights, row, col))

    return max_score


def scenic_score(heights, row, col):
    return (
        scenic_score_one_direction(heights, row, col, 1, 0)
        * scenic_score_one_direction(heights, row, col, -1, 0)
        * scenic_score_one_direction(heights, row, col, 0, 1)
        * scenic_score_one_direction(heights, row, col, 0, -1)
    )


def scenic_score_one_direction(heights, row, col, step_x, step_y):
    count = 0
    stop_height = heights[row, col]

    i = row + step_x
    j = col + step_y
    while i >= 0 and i < heights.shape[0] and j >= 0 and j < heights.shape[1]:
        count += 1

        height = heights[i][j]
        if height >= stop_height:
            break

        i += step_x
        j += step_y

    return count


if __name__ == "__main__":
    assert run("8-test.txt") == 8
    assert run("8-real.txt") == 527340
