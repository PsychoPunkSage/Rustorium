pub struct Allergies {
    // allergen: Vec<Option<Allergen>>,
    score: u8,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Allergen {
    Nil,
    Eggs,
    Peanuts,
    Shellfish,
    Strawberries,
    Tomatoes,
    Chocolate,
    Pollen,
    Cats,
}

impl Allergies {
    pub fn new(score: u32) -> Self {
        Allergies { score: score as u8 }
    }

    pub fn is_allergic_to(&self, allergen: &Allergen) -> bool {
        self.allergies().contains(allergen)
    }

    pub fn allergies(&self) -> Vec<Allergen> {
        let mut score = self.score;
        println!("Score: {}", score);
        let mut res = Vec::new();
        while score > 0 {
            let d = score % 2;
            score /= 2;
            res.insert(0, d);
        }

        println!("Res: {:?}", res);

        res.iter()
            .rev()
            .enumerate()
            .filter_map(|(index, &val)| {
                if val != 0 {
                    Some(index_to_allergen(index))
                } else {
                    None
                }
            })
            .collect::<Vec<Allergen>>()
    }
}

fn index_to_allergen(index: usize) -> Allergen {
    match index {
        0 => Allergen::Eggs,
        1 => Allergen::Peanuts,
        2 => Allergen::Shellfish,
        3 => Allergen::Strawberries,
        4 => Allergen::Tomatoes,
        5 => Allergen::Chocolate,
        6 => Allergen::Pollen,
        7 => Allergen::Cats,
        _ => Allergen::Nil,
    }
}
