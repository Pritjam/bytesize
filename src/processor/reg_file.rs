pub struct RegFile {
    registers: [u16; 8]
}

pub fn build_reg_file() -> RegFile {
    RegFile { registers: [0; 8] }
}

impl RegFile {
    pub fn read(&self, src_1: usize, src_2: usize) -> (u16, u16) {
        (self.registers[src_1], self.registers[src_2])
    }

    pub fn write(&mut self, dst_1: usize, dst_2: usize, wenable_1: bool, wenable_2: bool, wval_1: u16, wval_2: u16) {
        if wenable_1 {
            self.registers[dst_1] = wval_1;
        }

        if wenable_2 {
            self.registers[dst_2] = wval_2;
        }
    }

    pub fn generate_debug_string(&self) -> String {
        format!("\tax: {}\n\tbx: {}\n\tcx: {}\n\tdx: {}\n\tsi: {}\n\tdi: {}\n\tbp: {}\n\tsp: {}\n",
    self.registers[0], self.registers[1], self.registers[2], self.registers[3], self.registers[4], self.registers[5], self.registers[6], self.registers[7])
    }
}

