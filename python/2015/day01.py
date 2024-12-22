import itertools

chars = [1 if ch == "(" else -1 for ch in open("input01.txt").read()]

print(sum(chars))  # part 1 - 138
print(list(itertools.accumulate(chars)).index(-1) + 1)  # part 2 - 1771
