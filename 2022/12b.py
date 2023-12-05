import numpy as np


def parse(input_path):
    HEIGHTS = "abcdefghijklmnopqrstuvwxyz"

    input = open(input_path, "r")

    height_map = []
    end = None
    for i, line in enumerate(input.readlines()):
        line = line.strip()

        row = []
        for j, letter in enumerate(line):
            if letter == "S":
                row.append(HEIGHTS.find("a"))
            elif letter == "E":
                end = (i, j)
                row.append(HEIGHTS.find("z"))
            else:
                row.append(HEIGHTS.find(letter))

        height_map.append(row)

    height_map = np.array(height_map)

    return height_map, end


def test_node(
    height_map, current_height, distances, current_distance, new_wavefront, i, j
):
    if height_map[i, j] < current_height - 1:
        return

    if distances[i, j] > current_distance + 1:
        distances[i, j] = current_distance + 1
        new_wavefront.append((i, j))


def run(height_map, end):
    INF = 10000000

    distances = np.full_like(height_map, INF)
    distances[end] = 0

    wavefront = [end]
    while wavefront:
        new_wavefront = []
        for i, j in wavefront:
            dist = distances[i, j]
            height = height_map[i, j]

            # Right
            if i + 1 < height_map.shape[0]:
                test_node(height_map, height, distances, dist, new_wavefront, i + 1, j)

            # Up
            if j + 1 < height_map.shape[1]:
                test_node(height_map, height, distances, dist, new_wavefront, i, j + 1)

            # Left
            if i - 1 >= 0:
                test_node(height_map, height, distances, dist, new_wavefront, i - 1, j)

            # Down
            if j - 1 >= 0:
                test_node(height_map, height, distances, dist, new_wavefront, i, j - 1)

        wavefront = new_wavefront

    distances_to_min_heights = distances[height_map == 0]
    return min(distances_to_min_heights)


if __name__ == "__main__":
    assert run(*parse("12-test.txt")) == 29
    assert run(*parse("12-real.txt")) == 386
