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


def find_dependents(graph, node):
    dependents = set([node])
    to_find = set([node])
    while to_find:
        next_to_find = set()
        for k, v in graph.items():
            if not to_find.isdisjoint(v):
                dependents.add(k)
                next_to_find.add(k)
        to_find = next_to_find
    return dependents


def solve_op(x, op, a, b, solve_for):
    x = f"{x}'"

    match op, solve_for == a:
        case "+", True:
            return "-", x, b
        case "+", False:
            return "-", x, a
        case "-", True:
            return "+", x, b
        case "-", False:
            return "-", a, x
        case "*", True:
            return "/", x, b
        case "*", False:
            return "/", x, a
        case "/", True:
            return "*", x, b
        case "/", False:
            return "/", a, x
        case _:
            assert False


def run(input_path):
    values, exprs = parse(input_path)

    # Replace "root" with a binary equality op
    _, a, b = exprs["root"]
    exprs["root"] = ("=", a, b)

    # Start at "humn"
    # Move up the tree, inverting nodes as we go until we reach a binary equality node.
    # NOTE: We assume the structure is a tree and not a DAG.
    current = "humn"
    while True:
        res_node, op, a, b = next(
            (k, op, a, b) for k, (op, a, b) in exprs.items() if current in [a, b]
        )

        # Remove the old node
        exprs.pop(res_node)

        if op == "=":
            if current == a:
                exprs[f"{current}'"] = exprs[b]
            else:
                exprs[f"{current}'"] = exprs[a]
            break

        exprs[f"{current}'"] = solve_op(res_node, op, a, b, current)
        current = res_node

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

    return values["humn'"]


if __name__ == "__main__":
    assert run("21-test.txt") == 301
    assert run("21-real.txt") == 3093175982595
