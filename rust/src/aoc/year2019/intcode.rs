use crate::util::num;

const MAX_INSTRUCTION_LEN: usize = 5;

pub fn parse_program(program: &str) -> Vec<isize> {
    program
        .trim()
        .split(",")
        .map(|number| number.parse().unwrap())
        .collect()
}

#[derive(Default)]
pub struct Emulator {
    memory: Vec<isize>,
    input: Vec<isize>,
    output: Vec<isize>,
}

impl Emulator {
    pub fn new(program: Vec<isize>) -> Self {
        Self {
            memory: program,
            ..Default::default()
        }
    }

    pub fn with_input(program: Vec<isize>, input: Vec<isize>) -> Self {
        Self {
            memory: program,
            input,
            ..Default::default()
        }
    }

    pub fn run(&mut self) {
        let mut ip = 0;

        loop {
            let instruction = self.instruction(ip);
            let mut ip_modified = false;

            match instruction.opcode {
                Opcode::Add => {
                    let result = instruction.parameters[0] + instruction.parameters[1];
                    self.memory[instruction.parameters[2] as usize] = result;
                }
                Opcode::Multiply => {
                    let result = instruction.parameters[0] * instruction.parameters[1];
                    self.memory[instruction.parameters[2] as usize] = result;
                }
                Opcode::Input => {
                    let input = self.input.pop().unwrap();
                    self.memory[instruction.parameters[0] as usize] = input;
                }
                Opcode::Output => {
                    let output = self.memory[instruction.parameters[0] as usize];
                    self.output.push(output);
                }
                Opcode::JumpIfTrue => {
                    if instruction.parameters[0] != 0 {
                        ip_modified = true;
                        ip = instruction.parameters[1] as usize;
                    }
                }
                Opcode::JumpIfFalse => {
                    if instruction.parameters[0] == 0 {
                        ip_modified = true;
                        ip = instruction.parameters[1] as usize;
                    }
                }
                Opcode::LessThan => {
                    let result = instruction.parameters[0] < instruction.parameters[1];
                    self.memory[instruction.parameters[2] as usize] = result as isize;
                }
                Opcode::Equals => {
                    let result = instruction.parameters[0] == instruction.parameters[1];
                    self.memory[instruction.parameters[2] as usize] = result as isize;
                }
                Opcode::Halt => return,
                _ => unreachable!(),
            }

            ip += if ip_modified {
                0
            } else {
                instruction.parameters.len() + 1
            }
        }
    }

    pub fn memory(&self, address: usize) -> isize {
        *self.memory.get(address).unwrap_or(&0)
    }

    pub fn output(&self) -> &Vec<isize> {
        &self.output
    }

    fn instruction(&self, address: usize) -> Instruction {
        let digits = self.instruction_digits(address);
        let opcode = digits[MAX_INSTRUCTION_LEN - 2] * 10 + digits[MAX_INSTRUCTION_LEN - 1];
        let mut instruction = Instruction::new(Opcode::from(opcode));

        let parameters = match instruction.opcode {
            Opcode::Add | Opcode::Multiply | Opcode::LessThan | Opcode::Equals => vec![
                self.parameter(
                    address + 1,
                    ParameterMode::from(digits[MAX_INSTRUCTION_LEN - 3]),
                ),
                self.parameter(
                    address + 2,
                    ParameterMode::from(digits[MAX_INSTRUCTION_LEN - 4]),
                ),
                self.parameter(address + 3, ParameterMode::Output),
            ],
            Opcode::Input | Opcode::Output => {
                vec![self.parameter(address + 1, ParameterMode::Output)]
            }
            Opcode::JumpIfTrue | Opcode::JumpIfFalse => vec![
                self.parameter(
                    address + 1,
                    ParameterMode::from(digits[MAX_INSTRUCTION_LEN - 3]),
                ),
                self.parameter(
                    address + 2,
                    ParameterMode::from(digits[MAX_INSTRUCTION_LEN - 4]),
                ),
            ],
            Opcode::Halt => vec![],
            _ => unreachable!(),
        };

        instruction.parameters = parameters;
        instruction
    }

    fn instruction_digits(&self, address: usize) -> [u8; MAX_INSTRUCTION_LEN] {
        let mut digits = [0; MAX_INSTRUCTION_LEN];

        num::digits_rev(self.memory[address] as usize)
            .enumerate()
            .for_each(|(i, digit)| {
                digits[digits.len() - (i + 1)] = digit;
            });

        digits
    }

    fn parameter(&self, address: usize, mode: ParameterMode) -> isize {
        match mode {
            ParameterMode::Position => {
                let position = self.memory[address];
                self.memory[position as usize]
            }
            ParameterMode::Immediate | ParameterMode::Output => self.memory[address],
            _ => unreachable!(),
        }
    }
}

#[derive(Debug, Default)]
struct Instruction {
    opcode: Opcode,
    parameters: Vec<isize>,
}

impl Instruction {
    fn new(opcode: Opcode) -> Self {
        Self {
            opcode,
            ..Default::default()
        }
    }
}

#[derive(Debug, Default)]
enum Opcode {
    #[default]
    Add,
    Multiply,
    Input,
    Output,
    JumpIfTrue,
    JumpIfFalse,
    LessThan,
    Equals,
    Halt,
}

impl From<u8> for Opcode {
    fn from(value: u8) -> Self {
        match value {
            1 => Self::Add,
            2 => Self::Multiply,
            3 => Self::Input,
            4 => Self::Output,
            5 => Self::JumpIfTrue,
            6 => Self::JumpIfFalse,
            7 => Self::LessThan,
            8 => Self::Equals,
            99 => Self::Halt,
            _ => unreachable!(),
        }
    }
}

enum ParameterMode {
    Position,
    Immediate,
    Output,
}

impl From<u8> for ParameterMode {
    fn from(value: u8) -> Self {
        match value {
            0 => Self::Position,
            1 => Self::Immediate,
            _ => unreachable!(),
        }
    }
}
