use std::collections::HashSet;

fn indexes_of(val: Option<&u64>, i: usize, row: &[u64]) -> Vec<(usize, usize)> {
    match val {
        Some(&m) => row
            .iter()
            .enumerate()
            .filter_map(move |(j, &val)| if val == m { Some((i, j)) } else { None })
            .collect::<Vec<(usize, usize)>>(),
        None => vec![],
    }
}

pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut saddle_points = Vec::new();
    let max_candidates: HashSet<(usize, usize)> = input
        .iter()
        .enumerate()
        .flat_map(|(i, row)| indexes_of(row.iter().max(), i, row))
        .collect();

    max_candidates.iter().for_each(|&(i, j)| {
        let col: Vec<u64> = input.iter().map(|row| row[j]).collect();
        if Some(&input[i][j]) == col.iter().min() {
            saddle_points.push((i, j));
        }
    });
    saddle_points
}
