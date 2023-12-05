import re


def run(input_path):
    input = open(input_path, "r")

    stacks = []
    count = 0
    lines = iter(input.readlines())
    for line in lines:
        results = re.findall(r"((\[[A-Z]\])|(   ))( |$)", line)
        if not results:
            break

        for i, result in enumerate(results):
            if result[0] == "   ":
                continue

            while len(stacks) <= i:
                stacks.append([])

            stacks[i].append(result[0][1:2])
    next(lines)

    # Flip stacks
    for stack in stacks:
        stack.reverse()

    # Read instructions
    for line in lines:
        line = line.strip()

        result = re.search(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)", line)

        count = int(result.group(1))
        src = int(result.group(2))
        dst = int(result.group(3))

        for _ in range(count):
            stacks[dst - 1].append(stacks[src - 1].pop())

    return "".join(stack[-1] for stack in stacks)


if __name__ == "__main__":
    assert run("5-test.txt") == "CMZ"
    assert run("5-real.txt") == "JRVNHHCSJ"
