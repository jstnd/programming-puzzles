import time


def part1(list, num_set):
    for i in range(len(list)):
        if 2020 - list[i] in num_set:
            return list[i] * (2020 - list[i])


def part2(list, num_set):
    for i in range(len(list) - 1):
        for j in range(i + 1, len(list)):
            if 2020 - list[i] - list[j] in num_set:
                return list[i] * list[j] * (2020 - list[i] - list[j])


def main():
    with open("day1input.txt") as f:
        numbers = [int(n) for n in f.readlines()]

    # set for O(1) lookup
    number_set = set(n for n in numbers)

    # part 1
    start = time.time()
    print(f"Part 1: {part1(numbers, number_set)}; Time: {time.time() - start}")

    # part 2
    start = time.time()
    print(f"Part 2: {part2(numbers, number_set)}; Time: {time.time() - start}")


if __name__ == "__main__":
    main()
