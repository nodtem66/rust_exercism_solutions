fn is_stop_codon(codon: &[char]) -> bool {
    let str = codon.iter().collect::<String>();
    //println!("Codon: {}, is stop: {}", str, matches!(str.as_str(), "UAA" | "UAG" | "UGA"));
    matches!(str.as_str(), "UAA" | "UAG" | "UGA")
}

pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let mut proteins = Vec::new();
    let chars: Vec<char> = rna.chars().collect();
    let iter = chars.chunks(3).take_while(|&chunk| !is_stop_codon(chunk));
    for codon in iter {
        if codon.len() != 3 || codon.iter().any(|&c| !"AUCG".contains(c)) {
            return None;
        }
        let str = codon.iter().collect::<String>();
        let protein = match str.as_str() {
            "AUG" => Some("Methionine"),
            "UUU" | "UUC" => Some("Phenylalanine"),
            "UUA" | "UUG" => Some("Leucine"),
            "UCU" | "UCC" | "UCA" | "UCG" => Some("Serine"),
            "UAU" | "UAC" => Some("Tyrosine"),
            "UGU" | "UGC" => Some("Cysteine"),
            "UGG" => Some("Tryptophan"),
            _ => None,
        };
        if let Some(protein) = protein {
            proteins.push(protein);
        }
    }
    Some(proteins)
}
