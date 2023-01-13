pub fn extract(bit_string: usize, from: usize, length: usize) -> usize {
    (bit_string >> from) & ((1 << length) - 1)
}