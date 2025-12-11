use super::intcode;

pub fn part1(input: &str) -> isize {
    let mut program = intcode::parse_program(input);
    program[1] = 12;
    program[2] = 2;

    let mut emulator = intcode::Emulator::new(program);
    emulator.run();
    emulator.memory(0)
}

pub fn part2(input: &str) -> isize {
    let program = intcode::parse_program(input);

    for noun in 0..100 {
        for verb in 0..100 {
            let mut program = program.clone();
            program[1] = noun;
            program[2] = verb;

            let mut emulator = intcode::Emulator::new(program);
            emulator.run();

            if emulator.memory(0) == 19690720 {
                return 100 * noun + verb;
            }
        }
    }

    0
}
