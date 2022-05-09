extern crate byteorder;
use std::env;
use std::fs::File;
use std::io::Cursor;
use std::io::stdin;
use std::io::stdout;
use std::mem;
use std::process;
use std::io::prelude::*;
mod Machine;
mod Memory;
mod Registers;
mod Instructions;
mod parse_instructions;

use byteorder::{ReadBytesExt, BigEndian};

const AVAILABLE_ADDRESS: usize = 0;


// reads instructions from a file via command line
fn read_instructions(filename: &str) -> Vec<u32> {
    let mut f = File::open(filename).expect("no file found");
    let mut data = Vec::new();
    let mut instructions = Vec::new();

    match f.read_to_end(&mut data) {
        Ok(bytes) => {
            //println!("read {} bytes from {}", bytes, filename);

            for i in 0..data.len() / 4 {
                let index = i * 4;
                let buf = &data[index..index + 4];
                let mut dataReg = Cursor::new(buf);
                instructions.push(dataReg.read_u32::<BigEndian>().unwrap());
            }

            instructions
        },
        Err(e) => panic!(
            "Encountered error while reading from {}: {}", filename, e
        )
    }
}


fn main() {
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let instructions = read_instructions(filename);
    let mut machine = Machine::Machine::new(instructions);
    let mut instruction_num: usize = 0;
    //let mut instr_counter = 0;

    // let the machine run until it executes all instructions in instructions<>
    loop {
        let instruction = machine.get_instruction(instruction_num);
        instruction_num += 1;
        //instr_counter += 1;
        //println!("{}", instr_counter);

        match instruction.op {
            parse_instructions::Opcode::CMov => machine.cmov(instruction),
            parse_instructions::Opcode::Load => machine.load(instruction),
            parse_instructions::Opcode::Store => machine.store(instruction),
            parse_instructions::Opcode::Add => machine.add(instruction),
            parse_instructions::Opcode::Mul => machine.mul(instruction),
            parse_instructions::Opcode::Div => machine.div(instruction),
            parse_instructions::Opcode::Nand => machine.nand(instruction),
            parse_instructions::Opcode::Halt => process::exit(0),
            parse_instructions::Opcode::MapSegment => machine.mapsegment(instruction),
            parse_instructions::Opcode::UnmapSegment => machine.unmapsegment(instruction),
            parse_instructions::Opcode::Output => machine.output(instruction),
            parse_instructions::Opcode::Input => machine.input(instruction),
            parse_instructions::Opcode::LoadProgram => instruction_num = machine.loadprogram(instruction),
            parse_instructions::Opcode::LoadValue => machine.loadvalue(instruction),
            parse_instructions::Opcode::Err => panic!(
               "Unknown opcode for instruction {:?}", instruction
            )
        }
    }
}