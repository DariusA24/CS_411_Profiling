

#[derive(Debug)]
pub struct Instruction {
    pub op: crate::parse_instructions::Opcode,
    pub a: u32,
    pub b: Option<u32>,
    pub c: Option<u32>,
    pub value: Option<u32>
}

impl Instruction {
    pub fn new(instruction: u32) -> Instruction {
        let op = crate::parse_instructions::parse_op(instruction);
        let a = crate::parse_instructions::parse_a(instruction, &op);
        let b = crate::parse_instructions::parse_b(instruction, &op);
        let c = crate::parse_instructions::parse_c(instruction, &op);
        let value = crate::parse_instructions::parse_value(instruction, &op);

        Instruction {
            op,
            a,
            b,
            c,
            value
        }
    }
}