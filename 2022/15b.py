import numpy as np
import re

REGEX = "Sensor at x=(-?[0-9]+), y=(-?[0-9]+): closest beacon is at x=(-?[0-9]+), y=(-?[0-9]+)"


def parse(input_path):
    input = open(input_path, "r")
    input = input.read()

    matches = re.findall(REGEX, input)

    return np.array([[int(x) for x in m] for m in matches]).astype(np.int32)


def run(inputs, x_min, y_min, x_max, y_max):
    sensor_pos = inputs[:, 0:2]
    beacon_pos = inputs[:, 2:4]
    sensor_range = np.rint(
        np.linalg.norm(sensor_pos - beacon_pos, ord=1, axis=1)
    ).astype(np.int32)

    invisible = np.full(x_max - x_min + 1, True)

    # Very slow. Could be a lot faster with a hierarchical approach.
    # Split 4000000 row search space in half. See if half is completely visible, ignore it if not.
    # Continue down hierarchy.
    for y in range(y_min, y_max + 1):
        if y % 100 == 0:
            print(y)

        invisible.fill(True)

        # Set as no beacon if inside sensor range
        for i in range(sensor_pos.shape[0]):
            sensor_x, sensor_y = sensor_pos[i, :]
            nearest_distance = abs(sensor_y - y)

            spare_distance = sensor_range[i] - nearest_distance
            if spare_distance < 0:
                continue

            x_start = max(x_min, sensor_x - spare_distance)
            x_end = min(x_max, sensor_x + spare_distance)
            invisible[x_start - x_min : x_end - x_min + 1] = False

        if np.any(invisible):
            beacon_x = x_min + np.argwhere(invisible)
            assert beacon_x.size == 1
            return beacon_x[0][0] * 4000000 + y


def is_visible(check_pos, sensor_pos, sensor_range):
    return (
        np.rint(np.linalg.norm(check_pos - sensor_pos, ord=1)).astype(np.int32)
        <= sensor_range
    )


if __name__ == "__main__":
    assert run(parse("15-test.txt"), 0, 0, 20, 20) == 56000011
    assert run(parse("15-real.txt"), 0, 3429500, 4000000, 4000000) == 10996191429555
