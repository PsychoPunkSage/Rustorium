use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();

    for a in 1..(sum / 3) {
        for b in (a + 1)..(sum / 2) {
            let c = sum - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                triplets.insert([a, b, c]);
            }
        }
    }

    triplets
}
