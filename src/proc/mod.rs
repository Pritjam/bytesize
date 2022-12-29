mod alu;
mod reg_file;

pub struct Proc {
    accumulator: u16,
    flags: ProcFlags,
    regfile: reg_file::RegFile,
    instruction_pointer: u16,
}
#[derive(Copy, Clone)]
pub struct ProcFlags {
    negative: bool,
    zero: bool,
    carry: bool,
    overflow: bool,
}

pub fn build_proc() -> Proc{
    Proc {
        accumulator: 65534,
        flags: ProcFlags {
            negative: false,
            zero: false,
            carry: false,
            overflow: false,
        },
        regfile: reg_file::build_reg_file(),
        instruction_pointer: 0,
    }
}

impl Proc {

    pub fn fetch(&mut self) {
        
    }

    pub fn execute_instruction(&mut self, instruction: u8) {
        let op: alu::AluOp;
        match instruction {
            0..=3 => op = alu::AluOp::OpAdd,
            5 => op = alu::AluOp::OpSub,
            _ => op = alu::AluOp::OpNop,
        }
        let (result, flags) = alu::run_alu(self.accumulator, 1, self.flags, op);
        self.accumulator = result;
        self.flags = flags;
    }


    pub fn debug_str(&self) -> String {
        format!("Accumulator value: {}, IP value: {}", self.accumulator, self.instruction_pointer)
    }
}