def part1(dirs):
    current = [0, 0]
    visited = {(current[0], current[1])}

    for d in dirs:
        if d == "^":
            current[1] += 1
        elif d == "v":
            current[1] -= 1
        elif d == ">":
            current[0] += 1
        else:
            current[0] -= 1

        if (current[0], current[1]) not in visited:
            visited.add((current[0], current[1]))

    return len(visited)


def part2(dirs):
    santa = [0, 0]
    robo = [0, 0]
    visited = {(santa[0], santa[1])}

    for i, d in enumerate(dirs):
        if i % 2 == 0:
            if d == "^":
                santa[1] += 1
            elif d == "v":
                santa[1] -= 1
            elif d == ">":
                santa[0] += 1
            else:
                santa[0] -= 1

            if (santa[0], santa[1]) not in visited:
                visited.add((santa[0], santa[1]))
        else:
            if d == "^":
                robo[1] += 1
            elif d == "v":
                robo[1] -= 1
            elif d == ">":
                robo[0] += 1
            else:
                robo[0] -= 1

            if (robo[0], robo[1]) not in visited:
                visited.add((robo[0], robo[1]))

    return len(visited)


chars = [ch for ch in open("input03.txt").read()]

print(part1(chars))  # 2572
print(part2(chars))  # 2631
