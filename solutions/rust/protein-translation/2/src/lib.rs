pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::new();
    let bytes = rna.as_bytes();
    for codon in bytes.chunks(3) {
        let Ok(codon) = str::from_utf8(codon) else {
            return None;
        };
        let protein = match codon {
            "AUG" => Some("Methionine"),
            "UUU" | "UUC" => Some("Phenylalanine"),
            "UUA" | "UUG" => Some("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
            "UAU" | "UAC" => Some("Tyrosine"),
            "UGU" | "UGC" => Some("Cysteine"),
            "UGG" => Some("Tryptophan"),
            "UAA" | "UAG" | "UGA" => Some("STOP"),
            _ => None,
        };
        match protein {
            Some("STOP") => break,
            Some(p) => proteins.push(p),
            None => return None,
        }
    }
    Some(proteins)
}
