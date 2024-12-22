instr = [[cmd, int(n)] for cmd, n in map(str.split, open("input08.txt"))]


def execute():
    pc = acc = 0
    executed = set()
    while pc < len(instr) and pc not in executed:
        executed.add(pc)
        acc += instr[pc][1] if instr[pc][0] == "acc" else 0
        pc += instr[pc][1] if instr[pc][0] == "jmp" else 1

    return acc, pc == len(instr)


print(execute()[0])  # part 1 - 2034

for j, (ins, _) in enumerate(instr):  # part 2 - 672
    if ins != "acc":
        instr[j][0] = "jmp" if ins == "nop" else "nop"
        a, fixed = execute()
        instr[j][0] = ins
        if fixed:
            print(a)
            break
