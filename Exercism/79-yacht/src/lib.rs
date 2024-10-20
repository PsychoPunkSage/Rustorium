#[derive(Debug)]
pub enum Category {
    Ones,
    Twos,
    Threes,
    Fours,
    Fives,
    Sixes,
    FullHouse,
    FourOfAKind,
    LittleStraight,
    BigStraight,
    Choice,
    Yacht,
}

type Dice = [u8; 5];

pub fn score(dice: Dice, category: Category) -> u8 {
    let counts = dice.iter().fold([0; 7], |mut acc, &ele| {
        acc[ele as usize] += 1;
        acc
    });
    match category {
        Category::Ones => 1 * dice.iter().filter(|&ele| ele == &1).count() as u8,
        Category::Twos => 2 * dice.iter().filter(|&ele| ele == &2).count() as u8,
        Category::Threes => 3 * dice.iter().filter(|&ele| ele == &3).count() as u8,
        Category::Fours => 4 * dice.iter().filter(|&ele| ele == &4).count() as u8,
        Category::Fives => 5 * dice.iter().filter(|&ele| ele == &5).count() as u8,
        Category::Sixes => 6 * dice.iter().filter(|&ele| ele == &6).count() as u8,
        Category::FullHouse => {
            if counts.iter().any(|&count| count == 3) && counts.iter().any(|&count| count == 2) {
                dice.iter().sum()
            } else {
                0
            }
        }
        Category::FourOfAKind => {
            if counts.iter().any(|&count| count >= 4) {
                let pos = counts.iter().position(|&ele| ele >= 4).unwrap() as u8;
                pos * 4
            } else {
                0
            }
        }
        Category::LittleStraight => {
            if !counts.iter().any(|&ele| ele > 1) && dice.iter().all(|&num| num < 6) {
                30
            } else {
                0
            }
        }
        Category::BigStraight => {
            if !counts.iter().any(|&ele| ele > 1) && dice.iter().all(|&num| num > 1) {
                30
            } else {
                0
            }
        }
        Category::Choice => dice.iter().sum(),
        Category::Yacht => {
            if counts.iter().any(|&count| count == 5) {
                50
            } else {
                0
            }
        }
    }
}
