// use bitpack::bitpack; 
// extracts a range of bits from a u32
pub fn getu(value: u32, lsb: u32, n: u32) -> u32 {
    let field: u32 = ((1 << n) - 1) << lsb;
    (value & field) >> lsb
}

#[derive(Debug, PartialEq)]
pub enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
    Err
}

// methods for parsing the instructions.
pub fn parse_op(instruction: u32) -> Opcode {
    let op =  getu(instruction, 28, 4);//bitpack::getu(instruction as u64, 4, 28).try_into().unwrap();

    match op {
        0 => Opcode::CMov,
        1 => Opcode::Load,
        2 => Opcode::Store,
        3 => Opcode::Add,
        4 => Opcode::Mul,
        5 => Opcode::Div,
        6 => Opcode::Nand,
        7 => Opcode::Halt,
        8 => Opcode::MapSegment,
        9 => Opcode::UnmapSegment,
        10 => Opcode::Output,
        11 => Opcode::Input,
        12 => Opcode::LoadProgram,
        13 => Opcode::LoadValue,
        _ => Opcode::Err
    }
}

// gets the a value from the instruction
pub fn parse_a(instruction: u32, op: &Opcode) -> u32 {
//     match *op {
//         Opcode::LoadValue => bitpack::getu(instruction as u64, 3, 25).try_into().unwrap(),
//         _ => bitpack::getu(instruction as u64, 3, 6).try_into().unwrap()
//     }
    
    match *op {
        Opcode::LoadValue => getu(instruction, 25, 3),
        _ => getu(instruction, 6, 3)
    }
   
}

// gets the b value from the instruction
pub fn parse_b(instruction: u32, op: &Opcode) -> Option<u32> {
//     match *op {
//         Opcode::LoadValue => None,
//         _ => Some(bitpack::getu(instruction as u64, 3, 3).try_into().unwrap()) 
//     } 
    
    match *op {
        Opcode::LoadValue => None,
        _ => Some(getu(instruction, 3, 3))
    }
    
}

// gets the c value from the instruction
pub fn parse_c(instruction: u32, op: &Opcode) -> Option<u32> {
//     match *op {
//         Opcode::LoadValue => None,
//         _ => Some(bitpack::getu(instruction as u64, 3, 0).try_into().unwrap())
//     } 
    
    match *op {
        Opcode::LoadValue => None,
        _ => Some(getu(instruction, 0, 3))
    }
    
}

// gets the d value from the instruction
pub fn parse_value(instruction: u32, op: &Opcode) -> Option<u32> {

//     match *op {
//         Opcode::LoadValue => Some(bitpack::getu(instruction as u64, 25, 0).try_into().unwrap()),
//         _ => None
//     } 
    
    match *op {
        Opcode::LoadValue => Some(getu(instruction, 0, 25)),
        _ => None
    }
    
}

/* 
use bitpack::bitpack; 

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Opcode {
    CMov,
    Load,
    Store,
    Add,
    Mul,
    Div,
    Nand,
    Halt,
    MapSegment,
    UnmapSegment,
    Output,
    Input,
    LoadProgram,
    LoadValue,
    Error,
}

//Parsing instructions
pub fn parse_op(word: u32) -> Opcode {
let op_move = bitpack::getu(word as u64, 4, 28).try_into().unwrap(); 
match op_move {
    0 => Opcode::CMov,
    1 => Opcode::Load,
    2 => Opcode::Store,
    3 => Opcode::Add,
    4 => Opcode::Mul,
    5 => Opcode::Div,
    6 => Opcode::Nand,
    7 => Opcode::Halt,
    8 => Opcode::MapSegment,
    9 => Opcode::UnmapSegment,
    10 => Opcode::Output,
    11 => Opcode::Input,
    12 => Opcode::LoadProgram,
    13 => Opcode::LoadValue,
    _ => Opcode::Error,
}

}
//Parse the value a, depending on how the value is loaded
pub fn a_parse(word: u32, op: &Opcode) -> u32 {
    match *op {
        Opcode::LoadValue => bitpack::getu(word as u64, 3, 25).try_into().unwrap(),
        _ => bitpack::getu(word as u64, 3, 6).try_into().unwrap()
    }
}
//Parse the value b, depending on how the value is loaded
pub fn b_parse(word: u32, op: &Opcode) -> Option<u32> {
    match *op {
        Opcode::LoadValue => None,
        _ => Some(bitpack::getu(word as u64, 3, 3).try_into().unwrap()) 
    } 
}
//Parse the value c, depending on how the value is loaded
pub fn c_parse(word: u32, op: &Opcode) -> Option<u32>  {
    match *op {
        Opcode::LoadValue => None,
        _ => Some(bitpack::getu(word as u64, 3, 0).try_into().unwrap())
    } 
}
//Parse the value d, depending on how the value is loaded
pub fn val_parse(word: u32, op: &Opcode) -> Option<u32>  {
    match *op {
        Opcode::LoadValue => Some(bitpack::getu(word as u64, 25, 0).try_into().unwrap()),
        _ => None
    } 
}

*/
