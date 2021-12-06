nums = list(map(int, open("01.txt")))
print(sum(nums[i + 1] > nums[i] for i in range(len(nums) - 1)))  # part 1 - 1162
print(sum(nums[i + 3] + nums[i + 2] + nums[i + 1] > nums[i + 2] + nums[i + 1] + nums[i] for i in range(len(nums) - 3)))  # part 2 - 1190
