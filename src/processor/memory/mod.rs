
pub struct Mem {
    mem_array: Box<[u16; u16::MAX as usize + 1]>,
    // other members perhaps
}

impl Mem {
    pub fn mem(&mut self, address: usize, do_read: bool, do_write: bool, write_val: u16) -> Option<u16> {
        if do_read && do_write {
            unimplemented!("[FATAL] Cannot read and write simultaneously!");
        }

        if do_write {
            self.mem_array[address] = write_val;
            return None;
        } else {
            return Some(self.mem_array[address]);
        }
    }
}

pub fn build_mem() -> Mem {
    Mem {
        mem_array: Box::new([0; u16::MAX as usize + 1]),
    }
}
