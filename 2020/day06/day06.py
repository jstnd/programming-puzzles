inp = open("input06.txt").read().split("\n\n")
print(sum(len(set(g.replace("\n", ""))) for g in inp))  # part 1 - 6683
print(sum(len(set.intersection(*map(set, g.split("\n")))) for g in inp))  # part 2 - 3122
