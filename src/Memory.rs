use std::mem;

use crate::{Instructions::Instruction, AVAILABLE_ADDRESS};

#[derive(Debug, PartialEq, Clone)]
pub struct Memory {
    free_addresses: Vec<usize>,
    heap: Vec<Vec<u32>>
}

impl Memory {
    pub fn new(instructions: Vec<u32>) -> Memory {
        Memory {
            free_addresses: Vec::new(),
            heap: vec![instructions]
        }
    }

    // starts with a vec of zeros that has the given size
    // returns the address of the newly mapped segment
    pub fn mapsegment(&mut self, size: usize) -> usize {
        let zeros = vec![0_u32; size];

        if self.free_addresses.len() == 0 {
            self.heap.push(zeros);

            self.heap.len() - 1
        } else {
            let address = self.free_addresses.pop()
                .expect("no free addresses available");

            mem::replace(
                self.heap.get_mut(address)
                    .expect("memory was not previously allocated"),
                zeros
            );

            address
        }
    }

    // unmaps the segment at the given address.
    pub fn unmapsegment(&mut self, address: usize) {
        self.free_addresses.push(address);
        mem::replace(
            self.heap.get_mut(address)
                .expect("memory was not previously allocated"),
            Vec::new()
        );
    }

    // returns the contents of the memory at the given address if
    // it isn't empty
    pub fn get(&self, address: usize) -> Option<&Vec<u32>> {
        self.heap.get(address)
    }

    // get the instruction corresponding to the given instruction number
    pub fn get_instruction(&self, instruction_num: usize) -> Instruction {
        match self.heap.get(AVAILABLE_ADDRESS) {
            Some(program) => Instruction::new(program[instruction_num]),
            None => panic!("program was unallocated")
        }
    }

    // write a value into the given index at the given address
    pub fn set(&mut self, address: usize, index: usize, value: u32) {
        let memory = self.heap.get_mut(address)
            .expect("memory was unallocated");

        mem::replace(
            memory.get_mut(index)
                .expect("no value present at given index"),
            value
        );
    }

    // replace the program with the vector at the given address
    pub fn load(&mut self, address: usize) {
        let program = self.heap.get(address)
            .expect("no program at the given address")
            .clone();

        mem::replace(
            self.heap.get_mut(AVAILABLE_ADDRESS)
                .expect("no existing program"),
            program
        );
    }
}
