inp = list(map(int, open("in02.txt").read().split(",")))


def execute(mem):
    pc = 0
    op = mem[pc]
    while op != 99:
        if op == 1:
            mem[mem[pc + 3]] = mem[mem[pc + 1]] + mem[mem[pc + 2]]
        elif op == 2:
            mem[mem[pc + 3]] = mem[mem[pc + 1]] * mem[mem[pc + 2]]
        pc += 4
        op = mem[pc]

    return mem[0]


def part1():
    program = inp[:]
    program[1] = 12
    program[2] = 2
    return execute(program)


def part2():
    for n in range(100):
        for v in range(100):
            program = inp[:]
            program[1] = n
            program[2] = v
            if execute(program) == 19690720:
                return 100 * n + v


print(part1())  # part 1 - 3562624
print(part2())  # part 2 - 8298
