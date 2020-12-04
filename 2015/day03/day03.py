moves = {"^": (0, 1), "v": (0, -1), ">": (1, 0), "<": (-1, 0)}


def part1(dirs):
    return len(deliver(dirs))


def part2(dirs):
    return len(deliver(dirs[::2]) | deliver(dirs[1::2]))


def deliver(dirs):
    pos = (0, 0)
    visited = {pos}

    for d in dirs:
        visited.add(pos := tuple(map(sum, zip(pos, moves[d]))))

    return visited


chars = [ch for ch in open("input03.txt").read()]

print(part1(chars))  # 2572
print(part2(chars))  # 2631
