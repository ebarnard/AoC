MOVE_SCORES = {
    "X": 1,  # Rock
    "Y": 2,  # Paper
    "Z": 3,  # Scissors
}

LOSE_SCORE = 0
DRAW_SCORE = 3
WIN_SCORE = 6

PLAY_SCORES = {
    ("A", "X"): DRAW_SCORE,
    ("A", "Y"): WIN_SCORE,
    ("A", "Z"): LOSE_SCORE,
    ("B", "X"): LOSE_SCORE,
    ("B", "Y"): DRAW_SCORE,
    ("B", "Z"): WIN_SCORE,
    ("C", "X"): WIN_SCORE,
    ("C", "Y"): LOSE_SCORE,
    ("C", "Z"): DRAW_SCORE,
}


def run(input_path):
    input = open(input_path, "r")

    score = 0
    for line in input.readlines():
        line = line.strip()
        them, you = line.split(" ", maxsplit=2)

        score += MOVE_SCORES[you] + PLAY_SCORES[(them, you)]

    return score


if __name__ == "__main__":
    assert run("2-test.txt") == 15
    assert run("2-real.txt") == 12458
