def binary(low, high, chars):
    for c in chars:
        if c in "FL":
            high = (high + low) // 2
        else:
            low = (high + low) // 2 + 1

    return high


seats = list(map(str.strip, open("input05.txt")))

print(max(ids := set(binary(0, 127, s[:7]) * 8 + binary(0, 7, s[7:]) for s in seats)))  # part 1 - 951
# print(max(ids := set(int("".join("0" if c in "FL" else "1" for c in s), 2) for s in seats)))  # alternate part 1

print(next(i for i in range(min(ids), max(ids) + 1) if i not in ids))  # part 2 - 653
