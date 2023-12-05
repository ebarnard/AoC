MOVE_SCORES = {
    "A": 1,  # Rock
    "B": 2,  # Paper
    "C": 3,  # Scissors
}

OUTCOME_SCORES = {
    "X": 0,  # Lose
    "Y": 3,  # Draw
    "Z": 6,  # Win
}

SHOULD_PLAY = {
    ("A", "X"): "C",
    ("A", "Y"): "A",
    ("A", "Z"): "B",
    ("B", "X"): "A",
    ("B", "Y"): "B",
    ("B", "Z"): "C",
    ("C", "X"): "B",
    ("C", "Y"): "C",
    ("C", "Z"): "A",
}


def run(input_path):
    input = open(input_path, "r")

    score = 0
    for line in input.readlines():
        line = line.strip()
        them, outcome = line.split(" ", maxsplit=2)

        score += OUTCOME_SCORES[outcome] + MOVE_SCORES[SHOULD_PLAY[(them, outcome)]]

    return score


if __name__ == "__main__":
    assert run("2-test.txt") == 12
    assert run("2-real.txt") == 12683
