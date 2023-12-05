from graphlib import TopologicalSorter
import re

NUMBMER_REGEX = "([a-z]+): ([0-9]+)"
EXPR_REGEX = "([a-z]+): ([a-z]+) (\+|-|\*|/) ([a-z]+)"


def parse(input_path):
    input = open(input_path, "r")
    input = input.read()

    values = re.findall(NUMBMER_REGEX, input)
    exprs = re.findall(EXPR_REGEX, input)

    return {n: int(v) for n, v in values}, {n: (op, a, b) for n, a, op, b in exprs}


def run(input_path):
    values, exprs = parse(input_path)

    expr_graph = {k: {a, b} for k, (_, a, b) in exprs.items()}
    ts = TopologicalSorter(expr_graph)
    for n in ts.static_order():
        if n in values:
            continue

        op, a, b = exprs[n]
        va = values[a]
        vb = values[b]
        match op:
            case "+":
                value = va + vb
            case "-":
                value = va - vb
            case "*":
                value = va * vb
            case "/":
                value = va // vb
            case _:
                assert False
        values[n] = value

    return values["root"]


if __name__ == "__main__":
    assert run("21-test.txt") == 152
    assert run("21-real.txt") == 299983725663456
