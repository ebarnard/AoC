from dataclasses import dataclass


def run(input_path):
    input = open(input_path, "r")

    accumulator = [0]
    state = State(cycle=0, reg_x=1)
    for line in input.readlines():
        line = line.strip()

        parts = line.split(" ")
        instruction = parts[0]
        args = parts[1:]

        INSTRUCTIONS[instruction](
            state, lambda s: cycle_callback(s, accumulator), *args
        )

    return accumulator[0]


def cycle_callback(state, accumulator):
    cycle_offset = state.cycle - 20
    if cycle_offset % 40 == 0:
        accumulator[0] += state.cycle * state.reg_x


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
    assert run("10-test.txt") == 13140
    assert run("10-real.txt") == 14620
