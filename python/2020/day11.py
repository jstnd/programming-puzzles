def run(seats, part):
    seat_limit = 4 if part == 1 else 5
    lines = [r[:] for r in seats]
    while True:
        changed = 0
        copy = [r[:] for r in lines]
        for y, ln in enumerate(lines):
            for x, s in enumerate(ln):
                occ = 0
                for dy, dx in [(-1, -1), (-1, 0), (-1, 1), (0, -1), (0, 1), (1, -1), (1, 0), (1, 1)]:
                    if part == 2:
                        orig_dy = dy
                        orig_dx = dx

                        while 0 <= y + dy < len(lines) and 0 <= x + dx < len(lines[0]) and lines[y + dy][x + dx] == ".":
                            dy += orig_dy
                            dx += orig_dx

                    if 0 <= y + dy < len(lines) and 0 <= x + dx < len(lines[0]) and lines[y + dy][x + dx] == "#":
                        occ += 1

                if s == "L" and occ == 0:
                    copy[y][x] = "#"
                    changed += 1
                elif s == "#" and occ >= seat_limit:
                    copy[y][x] = "L"
                    changed += 1

        if changed == 0:
            return sum(r.count("#") for r in lines)
        else:
            lines = [r[:] for r in copy]


inp = [[ch for ch in ln.strip()] for ln in open("input11.txt")]
print(run(inp, 1))  # part 1 - 2354
print(run(inp, 2))  # part 2 - 2072
