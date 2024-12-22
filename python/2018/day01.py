from itertools import cycle

inp = list(map(int, open("in01.txt")))
print(sum(inp))  # part 1 - 442

f = 0
seen = set()
for c in cycle(inp):
    if f in seen:
        break
    seen.add(f)
    f += c
print(f)  # part 2 - 59908
