import itertools


def part1(chs):
    return sum(1 if c == "(" else -1 for c in chs)


def part2(chs):
    return list(itertools.accumulate([1 if c == "(" else -1 for c in chs])).index(-1) + 1

    # floor = 0
    # for i, c in enumerate(chs):
    #     floor += 1 if c == "(" else -1
    #     if floor == -1:
    #         return i + 1


chars = [ch for ch in open("input01.txt").read()]

print(part1(chars))  # 138
print(part2(chars))  # 1771
