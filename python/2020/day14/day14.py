import itertools

inp = [ln.strip().split(" = ") for ln in open("in14.txt")]


def bitmask1(bn, mask):
    for i, c in enumerate(mask):
        if c != "X":
            bn = bn[:i] + c + bn[i + 1:]
    return int(bn, 2)


def bitmask2(ma, mask):
    for i, c in enumerate(mask):
        if c == "1" or c == "X":
            ma = ma[:i] + c + ma[i + 1:]

    ps = ["".join(b) for b in itertools.product("01", repeat=ma.count("X"))]

    ads = []
    for p in ps:
        c = ma
        for b in p:
            c = c.replace("X", b, 1)
        ads.append(c)

    return ads


def run(part):
    mem = {}
    mask = ""
    for m in inp:
        if m[0] == "mask":
            mask = m[1]
        else:
            if part == 1:
                mem[m[0]] = bitmask1(f"{int(m[1]):036b}", mask)
            else:
                for a in bitmask2(f"{int(m[0][4:-1]):036b}", mask):
                    mem[a] = int(m[1])

    return sum(mem.values())


print(run(1))  # part 1 - 6559449933360
print(run(2))  # part 2 - 3369767240513
