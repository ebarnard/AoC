import numpy as np
import re

REGEX = "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)"


def parse(input_path):
    input = open(input_path, "r")
    input = input.read()

    matches = re.findall(REGEX, input)

    return np.array([[int(x) for x in m] for m in matches]).astype(np.int32)


def run(inputs, y):
    sensor_pos = inputs[:, 0:2]
    beacon_pos = inputs[:, 2:4]
    sensor_range = np.rint(
        np.linalg.norm(sensor_pos - beacon_pos, ord=1, axis=1)
    ).astype(np.int32)

    x_min = np.amin(sensor_pos[:, 0] - sensor_range)
    x_max = np.amax(sensor_pos[:, 1] + sensor_range)

    no_beacon = np.full(x_max - x_min + 1, False)

    # Set as no beacon if inside sensor range
    for i in range(sensor_pos.shape[0]):
        sensor_x, sensor_y = sensor_pos[i, :]
        nearest_distance = abs(sensor_y - y)

        spare_distance = sensor_range[i] - nearest_distance
        if spare_distance < 0:
            continue

        no_beacon[
            sensor_x - spare_distance - x_min : sensor_x + spare_distance - x_min + 1
        ] = True

    # Set as beacon if there is already a beacon there
    for i in range(beacon_pos.shape[0]):
        beacon_x, beacon_y = beacon_pos[i, :]
        if beacon_y == y:
            no_beacon[beacon_x - x_min] = False

    no_beacon_count = np.sum(no_beacon)
    return no_beacon_count


def is_visible(check_pos, sensor_pos, sensor_range):
    return (
        np.rint(np.linalg.norm(check_pos - sensor_pos, ord=1)).astype(np.int32)
        <= sensor_range
    )


if __name__ == "__main__":
    assert run(parse("15-test.txt"), 10) == 26
    assert run(parse("15-real.txt"), 2000000) == 6425133
