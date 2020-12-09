nums = list(map(int, open("input09.txt").read().split()))


def find(index, number):
    for i in range(index - 26, index - 1):
        for j in range(i + 1, index):
            if nums[i] + nums[j] == number:
                return True
    return False


print(next((f := nums[i]) for i in range(26, len(nums)) if not find(i, nums[i])))  # part 1

for i in range(nums.index(f)):  # part 2
    curr = []
    j = i
    while sum(curr) < f:
        curr.append(nums[j])
        j += 1

    if sum(curr) == f:
        print(curr[0] + curr[-1])
        break
