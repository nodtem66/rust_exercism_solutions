use std::collections::HashMap;

const VALID_NUCLEOTIDES: [char; 4] = ['A', 'C', 'G', 'T'];

fn expect_valid_dna(dna: &str) -> Result<(), char> {
    if let Some(x) = dna.chars().find(|c| !VALID_NUCLEOTIDES.contains(c)) {
        Err(x)
    } else {
        Ok(())
    }
}

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    expect_valid_dna(dna)?;
    expect_valid_dna(&nucleotide.to_string())?;
    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    expect_valid_dna(dna)?;
    let mut counts = HashMap::from([( 'A', 0), ('C', 0), ('G', 0), ('T', 0)]);
    for nucleotide in dna.chars() {
        counts.entry(nucleotide).and_modify(|c| *c += 1);
    }
    Ok(counts)
}
