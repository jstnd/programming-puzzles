inp = list(map(int, open("in01.txt")))
print(sum(m // 3 - 2 for m in inp))  # part 1 - 3297866

fuel = 0  # part 2
for m in inp:
    f = m // 3 - 2
    while f > 0:
        fuel += f
        f = f // 3 - 2

print(fuel)  # 4943923
