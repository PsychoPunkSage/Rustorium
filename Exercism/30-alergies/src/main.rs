mod lib;
use lib::{Allergen, Allergies};

fn main() {
    let allergies = Allergies::new(1);
    assert!(allergies.is_allergic_to(&Allergen::Eggs))
}
