nums = list(map(int, open("input01.txt")))
num_set = set(nums)  # set for O(1) lookup


def part1():
    for i in range(len(nums)):
        if 2020 - nums[i] in num_set:
            return nums[i] * (2020 - nums[i])


def part2():
    for i in range(len(nums) - 1):
        for j in range(i + 1, len(nums)):
            if 2020 - nums[i] - nums[j] in num_set:
                return nums[i] * nums[j] * (2020 - nums[i] - nums[j])


print(part1())  # 1019371
print(part2())  # 278064990
