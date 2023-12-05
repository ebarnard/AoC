def priority(letter):
    ORDER = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ"

    assert len(letter) == 1
    pos = ORDER.find(letter)
    assert pos >= 0
    return 1 + pos


def run(input_path):
    input = open(input_path, "r")

    sum_priority = 0
    for line in input.readlines():
        line = line.strip()

        letters = [l for l in line]
        assert len(letters) % 2 == 0

        midpoint = len(letters) // 2
        compartment_a = set(letters[:midpoint])
        compartment_b = set(letters[midpoint:])

        duplicates = compartment_a.intersection(compartment_b)
        assert len(duplicates) == 1

        sum_priority += priority(next(iter(duplicates)))

    return sum_priority


if __name__ == "__main__":
    assert run("3-test.txt") == 157
    assert run("3-real.txt") == 7903
