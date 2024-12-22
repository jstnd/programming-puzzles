nums = list(map(int, open("01.txt")))

# initial solutions
print(sum(nums[i + 1] > nums[i] for i in range(len(nums) - 1)))  # part 1 - 1162
print(sum(nums[i + 3] + nums[i + 2] + nums[i + 1] > nums[i + 2] + nums[i + 1] + nums[i] for i in range(len(nums) - 3)))  # part 2 - 1190

# alternate solutions
print(sum(a < b for a, b in zip(nums, nums[1:])))  # part 1 - 1162
# only have to compare first number of first window and last number of second window as: a + b + c < b + c + d == a < d
print(sum(a < b for a, b in zip(nums, nums[3:])))  # part 2 - 1190
