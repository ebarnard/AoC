import dataclasses
import re
from typing import Callable


MONKEY_REGEX = """Monkey ([0-9]+):
  Starting items: ([0-9, ]+)
  Operation: new = old ([\*\+]) ([0-9a-z]+)
  Test: divisible by ([0-9]+)
    If true: throw to monkey ([0-9]+)
    If false: throw to monkey ([0-9]+)"""


@dataclasses.dataclass
class Monkey:
    items: list[int]
    worry_op: Callable[[int], int]
    test_divisible_by: int
    if_true_monkey: int
    if_false_monkey: int


def parse(input_path):
    input = open(input_path, "r")
    input = input.read()

    matches = re.findall(MONKEY_REGEX, input, re.MULTILINE)

    monkeys = []
    for (
        index,
        items,
        op,
        op_val,
        test_divisible_by,
        if_true_monkey,
        if_false_monkey,
    ) in matches:
        items = [int(item) for item in items.split(", ")]

        if op == "*" and op_val == "old":
            worry_op = lambda x: x * x
        elif op == "*":
            worry_op = lambda x, v=int(op_val): x * v
        elif op == "+":
            worry_op = lambda x, v=int(op_val): x + v
        else:
            assert False

        monkeys.append(
            Monkey(
                items,
                worry_op,
                int(test_divisible_by),
                int(if_true_monkey),
                int(if_false_monkey),
            )
        )

    return monkeys


def run(monkeys):
    inspection_count = [0 for _ in monkeys]
    for _ in range(20):
        for (i, m) in enumerate(monkeys):
            assert m.if_true_monkey != i
            assert m.if_false_monkey != i

            items = m.items
            m.items = []
            for worry in items:
                inspection_count[i] += 1

                worry = m.worry_op(worry)
                worry = worry // 3
                if worry % m.test_divisible_by == 0:
                    monkeys[m.if_true_monkey].items.append(worry)
                else:
                    monkeys[m.if_false_monkey].items.append(worry)

    inspection_count.sort(reverse=True)
    return inspection_count[0] * inspection_count[1]


if __name__ == "__main__":
    assert run(parse("11-test.txt")) == 10605
    assert run(parse("11-real.txt")) == 121450
