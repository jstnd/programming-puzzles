from itertools import combinations

nums = list(map(int, open("input01.txt")))
num_set = set(nums)  # set for fast lookup
print(next(n * (2020 - n) for n in nums if 2020 - n in num_set))  # part 1 - 1019371
print(next(a * b * (2020 - a - b) for a, b in combinations(nums, 2) if 2020 - a - b in num_set))  # part 2 - 278064990
