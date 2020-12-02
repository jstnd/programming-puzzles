import time


def part1():
    file = open("day2input.txt", "r")
    valid = 0

    for ln in file:
        line = ln.replace(":", "").replace("-", " ").strip("\n").split(" ")
        if int(line[1]) >= line[3].count(line[2]) >= int(line[0]):
            valid += 1

    return valid


def part2():
    file = open("day2input.txt", "r")
    valid = 0

    for ln in file:
        temp = 0
        line = ln.replace(":", "").replace("-", " ").strip("\n").split(" ")
        if (line[3])[int(line[0]) - 1] == line[2]:
            temp += 1

        if (line[3])[int(line[1]) - 1] == line[2]:
            temp += 1

        if temp == 1:
            valid += 1

    return valid


def main():
    start = time.time()
    print(f"Part 1: {part1()}; Time: {time.time() - start}")

    start = time.time()
    print(f"Part 2: {part2()}; Time: {time.time() - start}")


if __name__ == "__main__":
    main()
