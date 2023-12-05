MARKER_LEN = 4


def run(line):
    for i in range(0, len(line) - MARKER_LEN + 1):
        chars = line[i:][:MARKER_LEN]
        assert len(chars) == MARKER_LEN

        unique = set((c for c in chars))
        if len(unique) == MARKER_LEN:
            return i + MARKER_LEN


if __name__ == "__main__":
    assert run("bvwbjplbgvbhsrlpgdmjqwftvncz") == 5
    assert run("nppdvjthqldpwncqszvftbrmjlhg") == 6
    assert run("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg") == 10
    assert run("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw") == 11

    assert run(open("6-real.txt", "r").readline().strip()) == 1361
