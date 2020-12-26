inp = [[(r1 := ((r0 := ln.replace("[", " ").replace("]", "").strip().split())[0]))[:-4], int(r1[-3:]), r0[1]] for ln in open("input04.txt")]


def part1():
    total = 0
    for r in inp:
        fr = [(lt, r[0].count(lt)) for lt in set(r[0].replace("-", ""))]
        fr.sort(key=lambda x: (-x[1], x[0]))
        total += r[1] if set(r[2]) == set(t[0] for t in fr[:5]) else 0
    return total


def part2():
    for r in inp:
        new = "".join(chr((ord(lt) - 97 + r[1]) % 26 + 97) for lt in r[0])
        if new.startswith("northpole"):
            return r[1]


print(part1())
print(part2())
