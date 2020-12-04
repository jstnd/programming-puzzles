def part1(pwds):
    valid = 0

    for p in pwds:
        if int(p[1]) >= p[3].count(p[2]) >= int(p[0]):
            valid += 1

    return valid


def part2(pwds):
    valid = 0

    for p in pwds:
        if ((p[3])[int(p[0]) - 1] == p[2]) + ((p[3])[int(p[1]) - 1] == p[2]) == 1:
            valid += 1

    return valid


lines = [ln.replace(":", "").replace("-", " ").split() for ln in open("input02.txt").readlines()]

print(part1(lines))  # 564
print(part2(lines))  # 325
