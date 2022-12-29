pub struct RegFile {
    registers: [u16; 8]
}

pub fn build_reg_file() -> RegFile {
    RegFile { registers: [0; 8] }
}

