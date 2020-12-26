import math

inp = [int((f := open("input13.txt").readlines())[0].strip()), *[int(n) if n.isnumeric() else 0 for n in f[1].split(",")]]


def part1():
    buses = inp[1:]
    for i, n in enumerate(buses):
        if n == 0:
            continue
        while buses[i] < inp[0]:
            buses[i] += n

    return ((m := min(i for i in buses if i != 0)) - inp[0]) * inp[buses.index(m) + 1]


def part2():
    def inverse(b, i):
        multi = 1
        while (b * multi) % i != 1:
            multi += 1
        return multi

    buses = inp[1:]
    m = math.prod(i for i in buses if i != 0)
    M = [m // i if i != 0 else i for i in buses]
    y = [inverse(M[i], n) if n != 0 else n for i, n in enumerate(buses)]
    return sum((n - i) * M[i] * y[i] for i, n in enumerate(buses) if n != 0) % m


print(part1())  # part 1 - 2092
print(part2())  # part 2 - 702970661767766
