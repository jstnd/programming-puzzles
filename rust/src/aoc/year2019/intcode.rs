pub struct Emulator {
    memory: Vec<usize>,
}

impl Emulator {
    pub fn new(program: Vec<usize>) -> Self {
        Self { memory: program }
    }

    pub fn run(&mut self) {
        let mut ip = 0;

        loop {
            match self.memory[ip] {
                1 => {
                    let address1 = self.memory[ip + 1];
                    let address2 = self.memory[ip + 2];
                    let output_address = self.memory[ip + 3];
                    self.memory[output_address] = self.memory[address1] + self.memory[address2];

                    ip += 4;
                }
                2 => {
                    let address1 = self.memory[ip + 1];
                    let address2 = self.memory[ip + 2];
                    let output_address = self.memory[ip + 3];
                    self.memory[output_address] = self.memory[address1] * self.memory[address2];

                    ip += 4;
                }
                99 => return,
                _ => unreachable!(),
            }
        }
    }

    pub fn memory(&self, address: usize) -> usize {
        *self.memory.get(address).unwrap_or(&0)
    }
}
