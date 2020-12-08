import math


def traverse(list, w, b, right, down):
    position = [0, 0]
    trees = 0

    while position[0] < b:
        trees += (list[position[0]])[position[1]] == "#"

        position[0] += down
        position[1] = (position[1] + right) % w

    return trees


lines = list(map(str.strip, open("input03.txt")))

width = len(lines[0])
bottom = len(lines)

print(traverse(lines, width, bottom, 3, 1))  # part 1 - 162
print(math.prod(traverse(lines, width, bottom, r, d) for r, d in [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)]))  # part 2 - 3064612320
