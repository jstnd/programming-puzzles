import functools

inp = [0] + (s := sorted(map(int, open("input10.txt")))) + [s[-1] + 3]
print((d := [y - x for x, y in zip(inp, inp[1:])]).count(1) * d.count(3))  # part 1 - 1755


@functools.lru_cache(maxsize=None)
def find(curr):
    return 1 if curr == inp[-1] else sum(find(curr + i) for i in range(1, 4) if curr + i in inp)


print(find(0))  # part 2 - 4049565169664
