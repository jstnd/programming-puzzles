def part1(pwds):
    return sum(int(p[1]) >= p[3].count(p[2]) >= int(p[0]) for p in pwds)


def part2(pwds):
    return sum(((p[3])[int(p[0]) - 1] == p[2]) + ((p[3])[int(p[1]) - 1] == p[2]) == 1 for p in pwds)


lines = [ln.replace(":", "").replace("-", " ").split() for ln in open("input02.txt").readlines()]

print(part1(lines))  # 564
print(part2(lines))  # 325
