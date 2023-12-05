import numpy as np


def parse(input_path):
    input = open(input_path, "r")
    lines = input.readlines()

    return [int(v) for v in lines]


def run(numbers):
    items = np.array([(i, v) for i, v in enumerate(numbers)])

    for i in range(items.shape[0]):
        print(i)

        j = np.argwhere(items[:, 0] == i)[0][0].item()
        v = items[j, 1]
        new_j = (j + v) % (items.shape[0] - 1)
        if new_j < 0:
            new_j += items.shape[0] - 1

        if new_j == j:
            pass
        if new_j > j:
            items[j:new_j, :] = items[j + 1 : new_j + 1, :]
            items[new_j] = [i, v]
        if new_j < j:
            items[new_j + 1 : j + 1, :] = items[new_j:j, :]
            items[new_j] = [i, v]

        # print(",".join(str(items[i, 1].item()) for i in range(items.shape[0])))

    j = np.argwhere(items[:, 1] == 0)[0][0].item()
    return (
        items[(j + 1000) % items.shape[0], 1]
        + items[(j + 2000) % items.shape[0], 1]
        + items[(j + 3000) % items.shape[0], 1]
    )


if __name__ == "__main__":
    assert run(parse("20-test.txt")) == 3
    assert run(parse("20-real.txt")) == 4224
