def priority(letter):
    ORDER = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

    assert len(letter) == 1
    pos = ORDER.find(letter)
    assert pos >= 0
    return 1 + pos


def run(input_path):
    input = open(input_path, "r")

    sum_priority = 0
    common_items = set()
    for i, line in enumerate(input.readlines()):
        line = line.strip()

        letters = (l for l in line)
        if i % 3 == 0:
            common_items.clear()
            common_items.update(letters)
        else:
            common_items.intersection_update(letters)

        if i % 3 == 2:
            assert len(common_items) == 1
            sum_priority += priority(next(iter(common_items)))

    return sum_priority


if __name__ == "__main__":
    assert run("3-test.txt") == 70
    assert run("3-real.txt") == 2548
