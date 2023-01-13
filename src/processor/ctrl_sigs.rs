#[derive(Default)]
pub struct ctrl_sigs {
    // Decode
    pub src1_sel: bool,
    pub valb_imm: bool, // if true, the ALU uses the immediate as VALB.

    // Execute
    pub set_cc: bool, // if true, the ALU generates condition codes and these codes are stored.

    // Memory
    pub mem_read: bool, // If true, a memory read occurs.
    pub mem_write: bool, // If true, a memory write occurs.

    // Writeback
    pub wval_from_mem: bool, // If true, the primary writeback value comes from memory (otherwise from ALU)
    pub w_enable_1: bool, // If true, writes back the primary writeback value to dest_1.
    pub w_enable_2: bool, // If true, writes back the secondary writeback value (ALU output) to dest_2. 
}

pub fn generate(opcode: super::instr::Opcodes) -> ctrl_sigs {
    use super::instr::Opcodes;
    let src2_sel: bool;
    let valb_imm: bool;

    // src2_sel:
    match opcode {
        Opcodes::LOADBO
        | Opcodes::LOADPRE
        | Opcodes::LOADPOST
        | Opcodes::STOREBO
        | Opcodes::STOREPRE
        | Opcodes::STOREPOST => src2_sel = true,
        _ => src2_sel = false,
    }

    // valb_imm:
    match opcode {
        Opcodes::ALURI
        | Opcodes::LOADBO
        | Opcodes::LOADPRE
        | Opcodes::LOADPOST
        | Opcodes::STOREBO
        | Opcodes::STOREPRE
        | Opcodes::STOREPOST
        | Opcodes::MOVH
        | Opcodes::MOVL => valb_imm = true,
        _ => valb_imm = false,
    }

    ctrl_sigs {
        src1_sel: src2_sel,
        valb_imm: valb_imm,
        set_cc: false,
        mem_read: false,
        mem_write: false,
        wval_from_mem: false,
        w_enable_1: false,
        w_enable_2: false,
    }
}
