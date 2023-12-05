def parse_range(range):
    start, end = range.split("-")
    return int(start), int(end)


def is_overlap(a_start, a_end, b_start, b_end):
    assert a_end >= a_start
    assert b_end >= b_start
    return a_end >= b_start and a_start <= b_end


def run(input_path):
    input = open(input_path, "r")

    count = 0
    for line in input.readlines():
        line = line.strip()

        a_section, b_section = line.split(",")
        a_start, a_end = parse_range(a_section)
        b_start, b_end = parse_range(b_section)

        if is_overlap(a_start, a_end, b_start, b_end) or is_overlap(
            b_start, b_end, a_start, a_end
        ):
            count += 1

    return count


if __name__ == "__main__":
    assert run("4-test.txt") == 4
    assert run("4-real.txt") == 878
