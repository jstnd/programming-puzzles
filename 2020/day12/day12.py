def part1(inst):
    pos = [0, 0]
    face = 90
    for d in inst:
        action = d[0]
        value = int(d[1:])
        if action in "NS":
            pos[1] += value if action == "N" else -value
        elif action in "EW":
            pos[0] += value if action == "E" else -value
        elif action in "LR":
            face = (face + value) % 360 if action == "R" else ((face - value) + 360) % 360
        else:
            pos[0] += value if face == 90 else -value if face == 270 else 0
            pos[1] += value if face == 0 else -value if face == 180 else 0

    return abs(pos[0]) + abs(pos[1])


def part2(inst):
    pos = [0, 0]
    wp = [10, 1]
    for d in inst:
        action = d[0]
        value = int(d[1:])
        if action in "NS":
            wp[1] += value if action == "N" else -value
        elif action in "EW":
            wp[0] += value if action == "E" else -value
        elif action in "LR":
            for i in range(value // 90):
                wp[0], wp[1] = -wp[1] if action == "L" else wp[1], wp[0] if action == "L" else -wp[0]
        else:
            pos[0] += value * wp[0]
            pos[1] += value * wp[1]

    return abs(pos[0]) + abs(pos[1])


inp = list(map(str.strip, open("input12.txt")))
print(part1(inp))  # part 1 - 590
print(part2(inp))  # part 2 - 42013

