from dataclasses import dataclass


def run(input_path):
    input = open(input_path, "r")

    state = State(cycle=0, reg_x=1)
    for line in input.readlines():
        line = line.strip()

        parts = line.split(" ")
        instruction = parts[0]
        args = parts[1:]

        INSTRUCTIONS[instruction](state, cycle_callback, *args)


def cycle_callback(state):
    pos = (state.cycle - 1) % 40
    if pos == 0:
        print()

    cursor = state.reg_x

    if abs(cursor - pos) <= 1:
        print("#", end="")
    else:
        print(".", end="")


@dataclass
class State:
    cycle: int
    reg_x: int


def op_noop(state, cycle_callback):
    state.cycle += 1
    cycle_callback(state)


def op_addx(state, cycle_callback, value):
    state.cycle += 1
    cycle_callback(state)
    state.cycle += 1
    cycle_callback(state)
    state.reg_x += int(value)


INSTRUCTIONS = {
    "addx": op_addx,
    "noop": op_noop,
}

if __name__ == "__main__":
    run("10-test.txt")
    run("10-real.txt")
