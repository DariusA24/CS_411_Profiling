#[derive(Debug)]
pub struct Registers {
    registers: [u32; 8]
}

impl Registers {
    pub fn new() -> Registers {
        Registers {
            registers: [0_u32; 8]
        }
    }

    // get the value at the given register.
    pub fn get(&self, register: usize) -> u32 {
        self.registers[register]
    }

    // set the value in the given register.
    pub fn set(&mut self, register: usize, value: u32) {
        self.registers[register] = value
    }
}