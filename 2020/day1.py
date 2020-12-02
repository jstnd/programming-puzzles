import time


def part1(list):
    for i in range(len(list) - 1):
        if 2020 - list[i] in list:
            return list[i] * (2020 - list[i])

    # for i in range(len(list) - 1):
    #     first = list[i]
    #     second = 2020 - first
    #     if second in list:
    #         print("First:", first)
    #         print("Second:", second)
    #         return first * second
    #
    #     # for j in range(1, len(list)):
    #     #     second = list[j]
    #     #     if first + second == 2020:
    #     #         print("First:", first)
    #     #         print("Second:", second)
    #     #         return first * second


def part2(list):
    for i in range(len(list) - 2):
        for j in range(i + 1, len(list) - 1):
            if 2020 - list[i] - list[j] in list:
                return list[i] * list[j] * (2020 - list[i] - list[j])

    #         for k in range(j + 1, len(list)):
    #             if list[i] + list[j] + list[k] == 2020:
    #                 return list[i] * list[j] * list[k]
    #
    # for i in range(len(list) - 2):
    #     first = list[i]
    #     for j in range(i + 1, len(list) - 1):
    #         second = list[j]
    #         third = 2020 - first - second
    #         if third in list:
    #             print("First:", first)
    #             print("Second:", second)
    #             print("Third:", third)
    #             return first * second * third
    #
    #         # for k in range(j + 1, len(list)):
    #         #     third = list[k]
    #         #     if first + second + third == 2020:
    #         #         print("First:", first)
    #         #         print("Second:", second)
    #         #         print("Third:", third)
    #         #         return first * second * third


def main():
    with open("day1input.txt") as f:
        numbers = [int(n) for n in f.readlines()]

    # numbers = []
    # f = open("day1input.txt", "r")
    # for n in f:
    #     numbers.append(int(n))

    # part 1
    start = time.time()
    print(f"Part 1: {part1(numbers)}; Time: {time.time() - start}")

    # part 2
    start = time.time()
    print(f"Part 2: {part2(numbers)}; Time: {time.time() - start}")


if __name__ == "__main__":
    main()
