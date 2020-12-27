from collections import Counter

inp = list(map(str.strip, open("06.txt")))


def run(part):
    f = [list(), list(), list(), list(), list(), list(), list(), list()]
    for m in inp:
        for i, c in enumerate(m):
            f[i].append(c)

    for s in f:
        c = Counter(s).most_common()
        yield c[0][0] if part == 1 else c[-1][0]


print("".join(run(1)))  # part 1 - umejzgdw
print("".join(run(2)))  # part 2 - aovueakv
