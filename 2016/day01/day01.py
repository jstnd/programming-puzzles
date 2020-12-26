inp = open("input01.txt").read().split(", ")


def part1():
    pos = [0, 0]
    face = 0
    for d in inp:
        value = int(d[1:])
        face = (face + 90) % 360 if d[0] == "R" else ((face - 90) + 360) % 360
        pos[0] += value if face == 90 else -value if face == 270 else 0
        pos[1] += value if face == 0 else -value if face == 180 else 0
    return abs(pos[0]) + abs(pos[1])


def part2():
    pos = [0, 0]
    face = 0
    visited = set()
    for d in inp:
        value = int(d[1:])
        face = (face + 90) % 360 if d[0] == "R" else ((face - 90) + 360) % 360
        for _ in range(value):
            if (pos[0], pos[1]) in visited:
                return abs(pos[0]) + abs(pos[1])
            visited.add((pos[0], pos[1]))
            pos[0] += 1 if face == 90 else -1 if face == 270 else 0
            pos[1] += 1 if face == 0 else -1 if face == 180 else 0


print(part1())  # part 1 - 301
print(part2())  # part 2 - 130
