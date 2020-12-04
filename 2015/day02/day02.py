import heapq
import math


def part1(boxes):
    return sum(2 * sum(a := [b[x] * b[y] for x, y in [(0, 1), (1, 2), (0, 2)]]) + min(a) for b in boxes)


def part2(boxes):
    return sum(2 * sum(heapq.nsmallest(2, b)) + math.prod(b) for b in boxes)


lines = [list(map(int, ln.split("x"))) for ln in open("input02.txt").read().splitlines()]

print(part1(lines))  # 1586300
print(part2(lines))  # 3737498
