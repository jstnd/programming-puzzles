
inp = [ln for ln in open("input06.txt").read().split("\n\n")]
print(sum(len(set(g.replace("\n", ""))) for g in inp))  # part 1
print(sum(len(set.intersection(*[set(p) for p in g.split("\n")])) for g in inp))  # part 2
