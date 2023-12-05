input = open("1-real.txt", "r")

calories = []
sum_calories = 0
for line in input.readlines():
    line = line.strip()

    if not line:
        calories.append(sum_calories)
        sum_calories = 0
    else:
        sum_calories += int(line)

calories.sort(reverse=True)
print(sum(calories[0:3]))
