pwds = [ln.replace(":", "").replace("-", " ").split() for ln in open("input02.txt").readlines()]

print(sum(int(p[1]) >= p[3].count(p[2]) >= int(p[0]) for p in pwds))  # part 1 - 564
print(sum(((p[3])[int(p[0]) - 1] == p[2]) + ((p[3])[int(p[1]) - 1] == p[2]) == 1 for p in pwds))  # part 2 - 325
