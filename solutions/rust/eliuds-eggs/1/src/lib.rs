pub fn egg_count(display_value: u32) -> usize {
    let mut count = 0;
    let mut residue = display_value;
    while residue > 0 {
        count += residue & 1;
        residue >>= 1;
    }
    count as usize
}
