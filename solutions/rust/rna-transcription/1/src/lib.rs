#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

const DNA_NUCLEOTIDES: &str = "ACGT";
const RNA_NUCLEOTIDES: &str = "UGCA";

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if let Some(i) = dna.find(|c| !DNA_NUCLEOTIDES.contains(c)) {
            Err(i)
        } else {
            Ok(Self(dna.to_string()))
        }
    }

    pub fn into_rna(self) -> Rna {
        let rna = self.0.chars().map(|c| match c {
            'G' => 'C',
            'C' => 'G',
            'T' => 'A',
            'A' => 'U',
            _ => unreachable!(),
        }).collect();
        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if let Some(i) = rna.find(|c| !RNA_NUCLEOTIDES.contains(c)) {
            Err(i)
        } else {
            Ok(Self(rna.to_string()))
        }
    }
}
