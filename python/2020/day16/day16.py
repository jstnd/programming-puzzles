import math

inp = [ln.strip() for ln in open("in16.txt") if ln != "\n"]
ranges = {(s := ln.split(": "))[0]: s[1].split(' or ') for ln in inp[:inp.index("your ticket:")]}
nearby = [list(map(int, t.split(","))) for t in inp[inp.index("nearby tickets:") + 1:]]

to_remove = set()  # for part 2


def invalid(i, n):
    for ra in ranges.values():
        for r in ra:
            nums = r.split("-")
            if int(nums[0]) <= n <= int(nums[1]):
                return False
    to_remove.add(i)
    return True


print(sum(n for i, t in enumerate(nearby) for n in t if invalid(i, n)))  # part 1 - 25059


def part2():
    nb = [t for i, t in enumerate(nearby) if i not in to_remove]
    pos = {i: set(ranges.keys()) for i in range(len(nb[0]))}
    for t in nb:
        for i, f in enumerate(t):
            v = set()
            for ra in ranges:
                for r in ranges[ra]:
                    nums = r.split("-")
                    if int(nums[0]) <= f <= int(nums[1]):
                        v.add(ra)
            pos[i] -= pos[i] - v

    sk = [k for k in sorted(pos, key=lambda k: len(pos[k]), reverse=True)]
    for i, k in enumerate(sk[:-1]):
        pos[k] = (pos[k] - pos[sk[i + 1]]).pop()
    pos[sk[-1]] = pos[sk[-1]].pop()

    return math.prod(int(v) for i, v in enumerate(inp[inp.index("your ticket:") + 1].split(",")) if pos[i].startswith("departure"))


print(part2())  # part 2 - 3253972369789
