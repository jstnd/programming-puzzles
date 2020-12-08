def part1(list, num_set):
    for i in range(len(list)):
        if 2020 - list[i] in num_set:
            return list[i] * (2020 - list[i])


def part2(list, num_set):
    for i in range(len(list) - 1):
        for j in range(i + 1, len(list)):
            if 2020 - list[i] - list[j] in num_set:
                return list[i] * list[j] * (2020 - list[i] - list[j])


numbers = list(map(int, open("input01.txt")))
number_set = set(numbers)  # set for O(1) lookup

print(part1(numbers, number_set))  # 1019371
print(part2(numbers, number_set))  # 278064990
