from hashlib import md5

inp = "ugkcyxxp"


def part1():
    n = 0
    for _ in range(8):
        while (h := md5((inp + str(n)).encode()).hexdigest())[:5] != "00000":
            n += 1
        n += 1
        yield h[5]


def part2():
    n = 0
    p = [""] * 8
    while "" in p:
        while (h := md5((inp + str(n)).encode()).hexdigest())[:5] != "00000":
            n += 1
        if h[5].isdigit() and int(h[5]) < 8 and p[int(h[5])] == "":
            p[int(h[5])] = h[6]
        n += 1

    return "".join(p)


print("".join(part1()))
print(part2())
