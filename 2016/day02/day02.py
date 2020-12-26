inp = list(map(str.strip, open("input02.txt")))


def run(part):
    kp = [["", "", "", "", ""],
          ["",  1,  2,  3, ""],
          ["",  4,  5,  6, ""],
          ["",  7,  8,  9, ""],
          ["", "", "", "", ""]] if part == 1 \
          else [["", "",  "",  "",  "", "", ""],
                ["", "",  "",   1,  "", "", ""],
                ["", "",   2,   3,   4, "", ""],
                ["",  5,   6,   7,   8,  9, ""],
                ["", "", "A", "B", "C", "", ""],
                ["", "",  "", "D",  "", "", ""],
                ["", "",  "",  "",  "", "", ""]]
    p = [2, 2] if part == 1 else [3, 1]  # 5
    code = []
    for ln in inp:
        for c in ln:
            p[0] += 1 if c == "D" and kp[p[0] + 1][p[1]] != "" else -1 if c == "U" and kp[p[0] - 1][p[1]] != "" else 0
            p[1] += 1 if c == "R" and kp[p[0]][p[1] + 1] != "" else -1 if c == "L" and kp[p[0]][p[1] - 1] != "" else 0
        code.append(kp[p[0]][p[1]])

    return "".join(map(str, code))


print(run(1))  # part 1 - 56983
print(run(2))  # part 2 - 8B8B1
