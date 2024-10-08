pub fn translate(rna: &str) -> Option<Vec<&str>> {
    let patt = rna
        .chars()
        .collect::<Vec<char>>()
        .chunks(3)
        .map(|chunk| chunk.iter().collect::<String>())
        .collect::<Vec<String>>();

    let ans = patt
        .iter()
        .map(|codon| match codon.as_str() {
            "AUG" => "Methionine",
            "UUU" | "UUC" => "Phenylalanine",
            "UUA" | "UUG" => "Leucine",
            "UCU" | "UCC" | "UCA" | "UCG" => "Serine",
            "UAU" | "UAC" => "Tyrosine",
            "UGU" | "UGC" => "Cysteine",
            "UGG" => "Tryptophan",
            "UAA" | "UAG" | "UGA" => "STOP",
            _ => "FALSE",
        })
        .collect::<Vec<&str>>();

    if ans.iter().filter(|&protein| *protein == "STOP").count() == 0
        && ans.iter().filter(|&protein| *protein == "FALSE").count() > 0
    {
        return None;
    } else {
        let index = ans
            .iter()
            .position(|&c| c == "STOP")
            .unwrap_or_else(|| ans.len());
        Some(ans[..index].to_vec())
    }
}
