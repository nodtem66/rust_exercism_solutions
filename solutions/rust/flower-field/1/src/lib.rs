pub fn annotate(garden: &[&str]) -> Vec<String> {
    // Initialize a 2D vector to hold counts of adjacent flowers
    let mut counts: Vec<Vec<u8>> = garden.iter().map(|s| s.chars().map(|_| 0).collect()).collect();
    // Read texts and count adjacent flowers
    for (i, row) in garden.iter().enumerate() {
        for (j, c) in row.chars().enumerate() {
            if c == '*' {
                if i >= 1 && j >= 1 {
                    counts[i - 1][j - 1] += 1;
                }
                if i >= 1 {
                    counts[i - 1][j] += 1;
                }
                if i >= 1 && j + 1 < row.len() {
                    counts[i - 1][j + 1] += 1;
                }
                if j >= 1 {
                    counts[i][j - 1] += 1;
                }
                if j + 1 < row.len() {
                    counts[i][j + 1] += 1;
                }
                if i + 1 < garden.len() && j >= 1 {
                    counts[i + 1][j - 1] += 1;
                }
                if i + 1 < garden.len() {
                    counts[i + 1][j] += 1;
                }
                if i + 1 < garden.len() && j + 1 < row.len() {
                    counts[i + 1][j + 1] += 1;
                }
            }
        }
    }
    // Construct the result based on counts and original garden
    garden.iter().enumerate().map(|(i, row)| {
        row.chars()
            .enumerate()
            .map(|(j, c)| {
                if c == '*' {
                    '*'
                } else if counts[i][j] > 0 {
                    (counts[i][j] + b'0') as char
                } else {
                    ' '
                }
            })
            .collect()
    }).collect() 
}
