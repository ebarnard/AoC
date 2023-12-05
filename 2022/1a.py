input = open("1-real.txt", "r")

max_calories = 0
sum_calories = 0
for line in input.readlines():
    line = line.strip()

    if not line:
        max_calories = max(max_calories, sum_calories)
        sum_calories = 0
    else:
        sum_calories += int(line)

print(max_calories)
