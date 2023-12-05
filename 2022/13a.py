from itertools import zip_longest


def parse(input_path):
    input = open(input_path, "r")
    lines = input.readlines()
    pairs = []

    lines.reverse()
    while lines:
        pairs.append((parse_line(lines.pop()), parse_line(lines.pop())))
        if lines:
            assert lines.pop().strip() == ""

    return pairs


def parse_line(line):
    line = line.strip()
    line = list(reversed(line))
    return parse_list(line)


def parse_list(input):
    assert input.pop() == "["

    values = []

    # Handle empty list
    if input[-1] == "]":
        input.pop()
        return values

    while True:
        if input[-1] == "[":
            values.append(parse_list(input))
        else:
            values.append(parse_int(input))

        sep_or_term = input.pop()
        if sep_or_term == "]":
            break
        assert sep_or_term == ","

    return values


def parse_int(input):
    value = ""
    while input[-1] in "0123456789":
        value += input.pop()

    assert value
    return int(value)


def run(pairs):
    sum = 0
    for i, (left, right) in enumerate(pairs):
        if cmp_packet(left, right) <= 0:
            print(1 + i, "in order")
            sum += 1 + i
        else:
            print(1 + i, "out of order")
    return sum


def cmp_packet(left, right):
    # Return true of left is less than or equal to right
    if isinstance(left, int) and isinstance(right, int):
        if left < right:
            return -1
        elif left == right:
            return 0
        else:
            return 1

    if isinstance(left, list) and isinstance(right, list):
        for l, r in zip_longest(left, right):
            # Left ran out of items => left < right
            if l is None:
                return -1

            # Right ran out of items => left > right
            if r is None:
                return 1

            cmp = cmp_packet(l, r)
            if cmp != 0:
                return cmp

        # Both ran out of items
        return 0

    if isinstance(left, list) and isinstance(right, int):
        return cmp_packet(left, [right])

    if isinstance(left, int) and isinstance(right, list):
        return cmp_packet([left], right)

    assert False


if __name__ == "__main__":
    assert run(parse("13-test.txt")) == 13
    assert run(parse("13-real.txt")) == 5208
