def traverse(list, w, b, right, down):
    position = [0, 0]
    trees = 0

    while position[0] < b:
        trees += (list[position[0]])[position[1]] == "#"

        position[0] += down
        position[1] = (position[1] + right) % w

    return trees


def part1(lns, w, b):
    return traverse(lns, w, b, 3, 1)


def part2(lns, w, b):
    return traverse(lns, w, b, 1, 1) * traverse(lns, w, b, 3, 1) * traverse(lns, w, b, 5, 1) * traverse(lns, w, b, 7, 1) * traverse(lns, w, b, 1, 2)


lines = [ln.strip() for ln in open("input03.txt").readlines()]

width = len(lines[0])
bottom = len(lines)

print(part1(lines, width, bottom))  # 162
print(part2(lines, width, bottom))  # 3064612320
