pub fn part1(input: &str) -> isize {
    let program = super::intcode::parse_program(input);
    let mut emulator = super::intcode::Emulator::with_input(program, vec![1]);
    emulator.run();
    *emulator.output().last().unwrap()
}

pub fn part2(input: &str) -> usize {
    0
}
