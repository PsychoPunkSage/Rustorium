use ch14_cartes::kinds::PrimaryColor;
use ch14_cartes::mix; // High-level Extraction... It was directly exported

fn main() {
    let red = PrimaryColor::Red;
    let yellow = PrimaryColor::Yellow;
    mix(red, yellow);
}
