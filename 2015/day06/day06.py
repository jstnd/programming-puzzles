def handle(cmds, part):
    lights = [[0 for x in range(1000)] for y in range(1000)]

    for c in cmds:
        coord1 = tuple(map(int, c[1].split(",")))  # (y,x)
        coord2 = tuple(map(int, c[2].split(",")))

        for y in range(coord1[0], coord2[0] + 1):
            for x in range(coord1[1], coord2[1] + 1):
                if c[0] == "on":
                    if part == 1:
                        lights[y][x] = 1
                    else:
                        lights[y][x] += 1
                elif c[0] == "off":
                    if part == 1:
                        lights[y][x] = 0
                    else:
                        lights[y][x] -= 1 if lights[y][x] > 0 else 0
                else:
                    if part == 1:
                        lights[y][x] = int(not lights[y][x])
                    else:
                        lights[y][x] += 2

    return sum(map(sum, lights))


commands = [ln.strip().replace("turn", "").replace("through", "").split() for ln in open("input06.txt").readlines()]

print(handle(commands, 1))  # part 1 - 543903
print(handle(commands, 2))  # part 2 - 14687245
