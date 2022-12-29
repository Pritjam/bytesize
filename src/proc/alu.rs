use crate::proc;

pub enum AluOp {
    OpAdd,
    OpSub,
    OpAnd,
    OpOr,
    OpXor,
    OpLsl,
    OpLsr,
    OpAsr,
    OpAdc,
    OpSbc,
    OpVadd(bool),
    OpVsub(bool),
    OpVlsl(bool),
    OpVlsr(bool),
    OpNop,
}


pub fn run_alu(val_a: u16, val_b: u16, current_flags: proc::ProcFlags, operation: AluOp) -> (u16, proc::ProcFlags) {
    let res: u16;
    let carry: bool;
    let overflow: bool;
    match operation {
        AluOp::OpAdd => {
            res = val_a.wrapping_add(val_b);
            carry = res < val_a || res < val_b;
            overflow = (val_a >> 15 == val_b >> 15) && (val_a >> 15 != val_b >> 15); 
            // evil conditional that basically evaluates:
            // if a > 0 && b > 0 && res <= 0 OR a < 0 && b < 0 && res >= 0
        },
        AluOp::OpSub => {
            let temp = (!val_b).wrapping_add(1);
            res = val_a.wrapping_sub(temp);
            carry = val_b > val_a && res > 0;
            overflow = (val_b >> 15 == res >> 15) && (val_a >> 15 != res >> 15);
            // TODO: Fix this flag business
            // evil conditional that basically evaluates:
            // if b > 0 && res > 0 && res <= 0 OR a < 0 && b < 0 && res >= 0 
        },
        AluOp::OpAnd => {
            res = val_a & val_b;
            carry = false;
            overflow = false;
        },
        AluOp::OpOr => {
            res = val_a | val_b;
            carry = false;
            overflow = false;
        },
        AluOp::OpXor => {
            res = val_a ^ val_b;
            carry = false;
            overflow = false;
        },
        AluOp::OpLsl => {
            res = val_a << val_b;
            carry = false;
            overflow = false;
        },
        AluOp::OpLsr => {
            res = val_a >> val_b;
            carry = false;
            overflow = false;
        },
        AluOp::OpAsr => {
            res = ((val_a as i16) >> val_b) as u16;
            carry = false;
            overflow = false;
        },
        AluOp::OpAdc => {
            res = val_a.wrapping_add(val_b.wrapping_add(current_flags.carry as u16));
            carry = res < val_a || res < val_b;
            overflow = (val_a >> 15 == val_b >> 15) && (val_a >> 15 != val_b >> 15); 
            // evil conditional that basically evaluates:
            // if a > 0 && b > 0 && res <= 0 OR a < 0 && b < 0 && res >= 0
        },
        AluOp::OpSbc => {
            let temp = (!val_b).wrapping_add(1 + (current_flags.carry as u16));
            res = val_a.wrapping_sub(temp);
            carry = val_b > val_a && res > 0;
            overflow = (val_b >> 15 == res >> 15) && (val_a >> 15 != res >> 15);
            // TODO: Fix this flag business
            // evil conditional that basically evaluates:
            // if b > 0 && res > 0 && res <= 0 OR a < 0 && b < 0 && res >= 0 
        },
        AluOp::OpNop => return (0, proc::ProcFlags{
            negative: current_flags.negative,
            zero: current_flags.zero,
            carry: current_flags.carry,
            overflow: current_flags.overflow,
        }), // TODO: This is kinda gross, i don't love this.
        _ => todo!(),
    }

    (res, proc::ProcFlags {
        negative: (res >> 15) == 1,
        zero: res == 0,
        carry: carry,
        overflow: overflow })
}