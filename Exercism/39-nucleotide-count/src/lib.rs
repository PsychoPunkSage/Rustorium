use std::collections::HashMap;

pub fn count(nucleotide: char, dna: &str) -> Result<usize, char> {
    if !check_validity(dna) || !check_validity(nucleotide.to_string().as_ref()) {
        return Err('X');
    }

    Ok(dna.chars().filter(|&c| c == nucleotide).count())
}

pub fn nucleotide_counts(dna: &str) -> Result<HashMap<char, usize>, char> {
    if !check_validity(dna) {
        return Err('X');
    }

    let mut counts = HashMap::new();
    for &nucleotide in ['A', 'C', 'G', 'T'].iter() {
        *counts.entry(nucleotide).or_insert(0) += dna.chars().filter(|&c| c == nucleotide).count();
    }

    Ok(counts)
}

fn check_validity(dna: &str) -> bool {
    dna.chars()
        .all(|c| c == 'A' || c == 'C' || c == 'T' || c == 'G')
}
