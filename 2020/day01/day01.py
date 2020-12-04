def part1(list, num_set):
    for i in range(len(list)):
        if 2020 - list[i] in num_set:
            return list[i] * (2020 - list[i])


def part2(list, num_set):
    for i in range(len(list) - 1):
        for j in range(i + 1, len(list)):
            if 2020 - list[i] - list[j] in num_set:
                return list[i] * list[j] * (2020 - list[i] - list[j])


numbers = [int(n) for n in open("input01.txt").readlines()]

# set for O(1) lookup
number_set = set(n for n in numbers)

print(part1(numbers, number_set))  # 1019371
print(part2(numbers, number_set))  # 278064990
