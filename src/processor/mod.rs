mod alu;
mod reg_file;
mod instr;
mod ctrl_sigs;
mod memory;

pub struct Proc {
    accumulator: u16,
    flags: ProcFlags,
    regfile: reg_file::RegFile,
    instruction_pointer: u16,
    insn: instr::Instruction,
    mem: memory::Mem,
}
#[derive(Copy, Clone)]
pub struct ProcFlags {
    negative: bool,
    zero: bool,
    carry: bool,
    overflow: bool,
}

impl ProcFlags {
    pub fn generate_debug_string(&self) -> String {
        format!("N: {} Z: {} C: {} V: {}", self.negative, self.zero, self.carry, self.overflow)
    }
}

pub fn build_proc() -> Proc {
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
        insn: instr::build_insn(),
        mem: memory::build_mem(),
    }
}

impl Proc {

    pub fn fetch(&mut self) {

        // fetch instr        
        self.insn.insnbits = self.mem.mem(self.instruction_pointer as usize, true, false, 0).unwrap();
        
    }

    pub fn decode(&mut self) {
        use crate::bits;
        // extract opcode
        let op_bits = bits::extract(self.insn.insnbits as usize, 11, 5); // extract
        self.insn.opcode = instr::OPCODE_LOOKUP[op_bits];

        // detailed opcode
        if let instr::Opcodes::ALURR = self.insn.opcode {
            self.insn.opcode = instr::ALU_RR_LOOKUP[bits::extract(self.insn.insnbits as usize, 7, 4)];
        }

        if let instr::Opcodes::ALURI = self.insn.opcode {
            self.insn.opcode = instr::ALU_RI_LOOKUP[bits::extract(self.insn.insnbits as usize, 8, 3)]
        }

        // extract imm
        let imm = 0;

        // generate control signals
        self.insn.control_sigs = ctrl_sigs::generate(self.insn.opcode);

        // select ALU operation

        // extract destinations and sources
        self.insn.dst1 = bits::extract(self.insn.insnbits as usize, 0, 3);
        self.insn.dst2 = bits::extract(self.insn.insnbits as usize, 3, 3);
        let src1 = bits::extract(self.insn.insnbits as usize, 0, 3);
        let src2 = bits::extract(self.insn.insnbits as usize, 3, 3);

        let (rval_a, rval_b) = self.regfile.read(src1, src2);

        // If src1_sel is set, this is a memory indexing instruction, so the first ALU source is the base index
        if self.insn.control_sigs.src1_sel {
            self.insn.val_a = rval_b;
            self.insn.mem_writeval = rval_a; // mem_writeval is the val to be written to memory, which came from Rt
        } else {
            self.insn.val_a = rval_a;
        }

        if self.insn.control_sigs.valb_imm {
            self.insn.val_b = imm;
        } else {
            self.insn.val_b = rval_b;
        }

    }

    pub fn execute(&mut self) {

    }


    fn decide_alu_op(opcode: instr::Opcodes) {

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


    fn debug_print(&self) {
        println!("Processor state:\n\tIP value: {}\n\tFlags: {}\n\tInstruction Bits: {}\n\nRegister File: {}", self.instruction_pointer, self.flags.generate_debug_string(), self.insn.insnbits, self.regfile.generate_debug_string())
    }
}