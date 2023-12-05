DIRECTION_STEPS = {
    "R": (1, 0),
    "U": (0, 1),
    "L": (-1, 0),
    "D": (0, -1),
}


def run(input_path):
    input = open(input_path, "r")

    head_pos = (0, 0)
    tail_pos = (0, 0)
    tail_visited = set([tail_pos])

    for line in input.readlines():
        line = line.strip()

        direction, distance = line.split(" ")
        step = DIRECTION_STEPS[direction]

        for _ in range(int(distance)):
            head_pos = (head_pos[0] + step[0], head_pos[1] + step[1])
            tail_pos = move_tail(head_pos, tail_pos)
            tail_visited.add(tail_pos)

    return len(tail_visited)


def move_tail(head_pos, tail_pos):
    # Distance from tail to head
    dx = head_pos[0] - tail_pos[0]
    dy = head_pos[1] - tail_pos[1]

    # Check that head and tail are in a valid position
    assert abs(dx) <= 2 and abs(dy) <= 2
    assert not (abs(dx) == 2 and abs(dy) == 2)

    # Check if tail is within one space of head
    if abs(dx) <= 1 and abs(dy) <= 1:
        # If so, tail does not move
        return tail_pos

    # Straight to left or right
    if abs(dx) == 2 and abs(dy) == 0:
        return (tail_pos[0] + dx // 2, tail_pos[1])

    # Straight above or below
    if abs(dx) == 0 and abs(dy) == 2:
        return (tail_pos[0], tail_pos[1] + dy // 2)

    # Diagonally more to left or right than to top or bottom
    if abs(dx) == 2 and abs(dy) == 1:
        return (tail_pos[0] + dx // 2, tail_pos[1] + dy)

    # Diagonally more to top or bottom than left or right
    if abs(dx) == 1 and abs(dy) == 2:
        return (tail_pos[0] + dx, tail_pos[1] + dy // 2)

    # This should cover all cases
    assert False


if __name__ == "__main__":
    assert run("9-test.txt") == 13
    assert run("9-real.txt") == 5960
