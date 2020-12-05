
def find_pair(string):
    for i in range(len(string) - 3):
        for j in range(i + 2, len(string) - 1):
            if string[i] == string[j] and string[i + 1] == string[j + 1]:
                return True

    return False


def find_repeat(string):
    for i in range(len(string) - 2):
        if string[i] == string[i + 2]:
            return True

    return False


strings = [ln.strip() for ln in open("input05.txt").readlines()]

print(len([s for s in strings if sum(s.count(c) for c in "aeiou") >= 3 and sum(s.count(c * 2) for c in "abcdefghijklmnopqrstuvwxyz") >= 1 and sum(s.count(c) for c in {"ab", "cd", "pq", "xy"}) == 0]))  # part 1 - 255

print(len([s for s in strings if find_pair(s) and find_repeat(s)]))  # part 2 - 55
