def binary(high, low, chars):
    for c in chars:
        if c == "F" or c == "L":
            high -= ((high - low) + 1) // 2
        else:
            low += ((high - low) + 1) // 2

    return high


seats = [ln.strip() for ln in open("input05.txt").readlines()]

ids = []
highest = 0
for p in seats:
    ids.append(seat := binary(127, 0, p[:7]) * 8 + binary(7, 0, p[7:]))
    if seat > highest:
        highest = seat

print(highest)

for i in range(ids[0], ids[-1]):
    if i not in ids:
        print(i)
        break
