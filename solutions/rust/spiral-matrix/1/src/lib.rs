// Iterate the coordinates (row, column) of a spiral matrix
// starting from (start, start) to (end, end) (inclusive) in a clockwise manner.
pub fn spiral_index(start: usize, end: usize) -> Vec<(usize, usize)> {
    let mut x = Vec::new();
    match end.cmp(&start) {
        std::cmp::Ordering::Less => x,
        std::cmp::Ordering::Equal => {
            x.push((start, start));
            x
        }
        std::cmp::Ordering::Greater => {
            for i in start..=end {
                x.push((start, i));
            }
            for i in start + 1..=end {
                x.push((i, end));
            }
            for i in (start..=end - 1).rev() {
                x.push((end, i));
            }
            for i in (start + 1..=end - 1).rev() {
                x.push((i, start));
            }
            // Recurse for inner square
            let y = spiral_index(start + 1, end - 1);
            // Append inner square coordinates
            x.extend(y);
            x
        }
    }
}

pub fn spiral_matrix(size: u32) -> Vec<Vec<u32>> {
    let size = size as usize;
    let mut matrix = vec![vec![0; size]; size];
    let coords = if size > 0 { spiral_index(0, size - 1) } else { vec![] };
    for (i, (row, col)) in coords.into_iter().enumerate() {
        matrix[row][col] = i as u32 + 1;
    }
    matrix
}
