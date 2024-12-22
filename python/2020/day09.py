import itertools

nums = [int(n) for n in open("input09.txt")]


def find(index, number):
    for c in itertools.combinations(nums[index - 25: index], 2):
        if c[0] + c[1] == number:
            return True
    return False


print(next((fin := (i, n))[1] for i, n in enumerate(nums[25:], start=25) if not find(i, n)))  # part 1 - 36845998

for i in range(fin[0]):  # part 2 - 4830226
    curr = []
    j = i
    while sum(curr) < fin[1]:
        curr.append(nums[j])
        j += 1

    if sum(curr) == fin[1]:
        print(min(curr) + max(curr))
        break
