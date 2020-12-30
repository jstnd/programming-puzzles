from itertools import combinations

inp = list(map(str.strip, open("in02.txt")))
print(sum(any(s.count(l) == 2 for l in set(s)) for s in inp) * sum(any(s.count(l) == 3 for l in set(s)) for s in inp))  # part 1 - 5727
print(next(a[:l[0]] + a[l[0] + 1:] for a, b in combinations(inp, 2) if len(l := list(j for j, c in enumerate(zip(a, b)) if c[0] != c[1])) == 1))  # part 2 - uwfmdjxyxlbgnrotcfpvswaqh
