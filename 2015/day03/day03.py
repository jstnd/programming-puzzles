moves = {"^": (0, 1), "v": (0, -1), ">": (1, 0), "<": (-1, 0)}


def part1(dirs):
    current = (0, 0)
    visited = {current}

    for d in dirs:
        visited.add(current := tuple(map(sum, zip(current, moves[d]))))

    return len(visited)


def part2(dirs):
    santa = (0, 0)
    robo = (0, 0)
    visited = {santa}

    for i, d in enumerate(dirs):
        if i % 2 == 0:
            visited.add(santa := tuple(map(sum, zip(santa, moves[d]))))
        else:
            visited.add(robo := tuple(map(sum, zip(robo, moves[d]))))

    return len(visited)


chars = [ch for ch in open("input03.txt").read()]

print(part1(chars))  # 2572
print(part2(chars))  # 2631
