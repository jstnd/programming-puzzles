from itertools import combinations

inp = list(map(str.strip, open("in02.txt")))
print(sum(any(s.count(l) == 2 for l in set(s)) for s in inp) * sum(any(s.count(l) == 3 for l in set(s)) for s in inp))  # part 1 - 5727
print(next("".join(l) for a, b in combinations(inp, 2) if len(l := list(c for c, d in zip(a, b) if c == d)) == len(a) - 1))  # part 2 - uwfmdjxyxlbgnrotcfpvswaqh
