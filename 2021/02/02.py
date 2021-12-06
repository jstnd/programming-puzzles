commands = list(map(str.strip, open("02.txt")))


def part1():
    position, depth = 0, 0
    for command in commands:
        match command.split():
            case ["forward", num]: position += int(num)
            case ["down", num]: depth += int(num)
            case ["up", num]: depth -= int(num)

    return position * depth


def part2():
    position, depth, aim = 0, 0, 0
    for command in commands:
        match command.split():
            case ["forward", num]:
                position += int(num)
                depth += aim * int(num)
            case ["down", num]: aim += int(num)
            case ["up", num]: aim -= int(num)

    return position * depth


print(part1())  # part 1 - 1962940
print(part2())  # part 2 - 1813664422
