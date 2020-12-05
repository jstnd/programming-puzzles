import heapq
import math

boxes = [list(map(int, ln.split("x"))) for ln in open("input02.txt").read().splitlines()]

print(sum(2 * sum(a := [b[x] * b[y] for x, y in [(0, 1), (1, 2), (0, 2)]]) + min(a) for b in boxes))  # part 1 - 1586300
print(sum(2 * sum(heapq.nsmallest(2, b)) + math.prod(b) for b in boxes))  # part 2 - 3737498
