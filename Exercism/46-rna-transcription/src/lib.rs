#[derive(Debug, PartialEq, Eq)]
pub struct Dna(String);

#[derive(Debug, PartialEq, Eq)]
pub struct Rna(String);

impl Dna {
    pub fn new(dna: &str) -> Result<Dna, usize> {
        if dna.chars().all(|c| matches!(c, 'A' | 'C' | 'G' | 'T')) {
            Ok(Dna(dna.to_string()))
        } else {
            Err(dna
                .chars()
                .position(|c| !matches!(c, 'A' | 'C' | 'G' | 'T'))
                .unwrap_or(dna.len()))
        }
    }

    pub fn into_rna(self) -> Rna {
        let dna = self.0;
        let rna = dna
            .chars()
            .map(|c| match c {
                'A' => 'U',
                'C' => 'G',
                'G' => 'C',
                'T' => 'A',
                _ => panic!("Invalid DNA character"),
            })
            .collect();
        Rna(rna)
    }
}

impl Rna {
    pub fn new(rna: &str) -> Result<Rna, usize> {
        if rna.chars().all(|c| matches!(c, 'A' | 'C' | 'G' | 'U')) {
            Ok(Rna(rna.to_string()))
        } else {
            Err(rna
                .chars()
                .position(|c| !matches!(c, 'A' | 'C' | 'G' | 'U'))
                .unwrap_or(rna.len()))
        }
    }
}
