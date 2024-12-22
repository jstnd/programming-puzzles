from itertools import combinations

nums = list(map(int, open("input01.txt")))
num_set = set(nums)  # set for fast lookup
print(next(a * b for a in nums if (b := 2020 - a) in num_set))  # part 1 - 1019371
print(next(a * b * c for a, b in combinations(nums, 2) if (c := 2020 - a - b) in num_set))  # part 2 - 278064990
