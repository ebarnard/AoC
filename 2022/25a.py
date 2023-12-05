import math

SYM_TO_NUM = {
    "=": -2,
    "-": -1,
    "0": 0,
    "1": 1,
    "2": 2,
}

NUM_TO_SYM = {v: k for k, v in SYM_TO_NUM.items()}


def parse_line(line):
    line = line.strip()
    value = 0
    for c in line:
        value *= 5
        value += SYM_TO_NUM[c]

    return value


def to_base5(value):
    max_pow5 = math.floor(math.log(value, 5))
    digits = []
    acc = 0
    for pow5 in reversed(range(max_pow5 + 1)):
        acc *= 5
        digit = value // 5**pow5 - acc
        digits.append(digit)
        acc += digit

    return digits


def base5_to_snafu(digits):
    # Start from low digits
    reversed_digits = list(reversed(digits))
    i = 0
    while i < len(reversed_digits):
        assert reversed_digits[i] >= 0 and reversed_digits[i] <= 5

        if reversed_digits[i] >= 3:
            if i == len(reversed_digits) - 1:
                reversed_digits.append(0)

        if reversed_digits[i] == 3:
            reversed_digits[i + 1] += 1
            reversed_digits[i] = -2
        elif reversed_digits[i] == 4:
            reversed_digits[i + 1] += 1
            reversed_digits[i] = -1
        elif reversed_digits[i] == 5:
            reversed_digits[i + 1] += 1
            reversed_digits[i] = 0

        i += 1

    reversed_digits.reverse()
    return reversed_digits


def snafu_to_str(digits):
    string = ""
    for digit in digits:
        assert digit >= -2 and digit <= 2
        string += NUM_TO_SYM[digit]
    return string


def run(input_path):
    input = open(input_path, "r")

    total = 0
    for line in input.readlines():
        value = parse_line(line)
        total += value

    base5_digits = to_base5(total)
    snafu_digits = base5_to_snafu(base5_digits)
    return snafu_to_str(snafu_digits)


if __name__ == "__main__":
    assert parse_line("1121-1110-1=0") == 314159265

    assert run("25-test.txt") == "2=-1=0"
    assert run("25-real.txt") == "121=2=1==0=10=2-20=2"
