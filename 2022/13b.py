import functools
from itertools import zip_longest


def parse(input_path):
    input = open(input_path, "r")
    lines = input.readlines()
    return [parse_line(l.strip()) for l in lines if l.strip() != ""]


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


def run(packets):
    packets.append([[2]])
    packets.append([[6]])

    packets.sort(key=functools.cmp_to_key(cmp_packet))

    i = packets.index([[2]])
    j = packets.index([[6]])

    return (1 + i) * (1 + j)


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
    assert run(parse("13-test.txt")) == 140
    assert run(parse("13-real.txt")) == 25792
