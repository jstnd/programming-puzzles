from collections import Counter

inp = {(b := ln.replace(" bags", "").replace(" bag", "").split(" contain "))[0]: {c[2::]: int(c[0]) for c in b[1].strip().strip(".").split(", ") if c != "no other"} for ln in open("input07.txt")}


def part1(prev):
    bags = set()
    while len(prev) > 0:
        prev = set(b for b in inp for p in prev if p in inp[b])
        bags |= prev
    return len(bags)


def part2(prev):
    total = 0
    while len(prev) > 0:
        total += sum(sum(inp[p][c] for c in inp[p]) * prev[p] for p in prev)
        prev = sum((Counter({c: prev[p] * inp[p][c] for c in inp[p]}) for p in prev), Counter())
    return total


start = {"shiny gold": 1}

print(part1(start))  # 121
print(part2(start))  # 3805
