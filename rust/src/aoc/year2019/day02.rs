use super::intcode;

pub fn part1(input: &str) -> usize {
    let mut program: Vec<usize> = input
        .trim()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

    program[1] = 12;
    program[2] = 2;

    let mut emulator = intcode::Emulator::new(program);
    emulator.run();
    emulator.memory(0)
}

pub fn part2(input: &str) -> usize {
    let program: Vec<usize> = input
        .trim()
        .split(',')
        .map(|number| number.parse().unwrap())
        .collect();

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
