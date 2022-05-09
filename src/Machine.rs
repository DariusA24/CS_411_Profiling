use std::io::{stdout, Write, stdin, Read};

use crate::{Registers::Registers, Memory::Memory, Instructions::Instruction, AVAILABLE_ADDRESS};


#[derive(Debug)]
pub struct Machine {
    memory: Memory,
    registers: Registers
}

impl Machine {
    pub fn new(instructions: Vec<u32>) -> Machine {
        Machine {
            memory: Memory::new(instructions),
            registers: Registers::new()
        }
    }

    pub fn get_instruction(&self, pc: usize) -> Instruction {
        self.memory.get_instruction(pc)
    }

    pub fn cmov(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        if self.registers.get(c) != 0 {
            let value = self.registers.get(b);
            self.registers.set(a, value);
        }
    }

    pub fn load(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.registers.get(b) as usize;

        let array = self.memory.get(address)
            .expect("found unallocated array at the given address");
        let index = self.registers.get(c) as usize;

        let value = array[index];

        self.registers.set(a, value);
    }

    pub fn store(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.registers.get(a) as usize;
        let index = self.registers.get(b) as usize;
        let value = self.registers.get(c);

        self.memory.set(address, index, value);
    }

    pub fn add(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = self.registers.get(b).wrapping_add(self.registers.get(c));

        self.registers.set(a, value);
    }

    pub fn mul(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = self.registers.get(b).wrapping_mul(self.registers.get(c));

        self.registers.set(a, value);
    }

    pub fn div(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = self.registers.get(b).wrapping_div(self.registers.get(c));
        self.registers.set(a, value);
    }

    pub fn nand(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let value = !(self.registers.get(b) & self.registers.get(c));

        self.registers.set(a, value);
    }

    pub fn mapsegment(&mut self, instruction: Instruction) {
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let size = self.registers.get(c) as usize;

        let address = self.memory.mapsegment(size);

        self.registers.set(b, address as u32);
    }

    pub fn unmapsegment(&mut self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;
        let address = self.registers.get(c) as usize;

        self.memory.unmapsegment(address);
    }

    pub fn output(&self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;

        let value = self.registers.get(c);

        let byte = value as u8;
        stdout().write(&[byte]).unwrap();
    }

    pub fn input(&mut self, instruction: Instruction) {
        let c = instruction.c.unwrap() as usize;

        match stdin().bytes().next().unwrap() { // EOF will be None
            Ok(value) => {
                if value as char == '\n' {
                    self.registers.set(c, std::u32::MAX);
                } else {
                    self.registers.set(c, value as u32);
                }
            },
            Err(e) => panic!("Encountered error while reading input: {}", e)
        }
    }

    pub fn loadprogram(&mut self, instruction: Instruction) -> usize {
        let b = instruction.b.unwrap() as usize;
        let c = instruction.c.unwrap() as usize;

        let address = self.registers.get(b) as usize;

        if address != AVAILABLE_ADDRESS {
            self.memory.load(address);
        }

        self.registers.get(c) as usize
    }

    pub fn loadvalue(&mut self, instruction: Instruction) {
        let a = instruction.a as usize;
        let value = instruction.value.unwrap();

        self.registers.set(a, value);
    }
}