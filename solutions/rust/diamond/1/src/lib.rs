pub fn get_diamond(c: char) -> Vec<String> {
    let n: usize = (2 * (c as u8 - b'A') + 1) as usize;
    let mut diamond = Vec::new();
    let mid: usize = (n - 1) / 2;
    for i in 0..n {
        let c = if i <= mid { i } else { n - i - 1 };
        let mut line = vec![' '; n];
        let left_idx = mid - c;
        let right_idx = mid + c;
        line[left_idx] = (b'A' + c as u8) as char;
        line[right_idx] = (b'A' + c as u8) as char;
        diamond.push(line.iter().collect())
    }
    diamond
}
