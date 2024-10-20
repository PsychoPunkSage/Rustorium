use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    // let mut triplets = HashSet::new();

    //// NOOB
    // for a in 1..(sum / 3) {
    //     for b in (a + 1)..(sum / 2) {
    //         let c = sum - a - b;
    //         if a.pow(2) + b.pow(2) == c.pow(2) {
    //             triplets.insert([a, b, c]);
    //         }
    //     }
    // }

    //// Mid
    // (1..sum / 3).for_each(|a| {
    //     ((a + 1)..sum / 2).for_each(|b| {
    //         let c = sum - a - b;
    //         if a.pow(2) + b.pow(2) == c.pow(2) {
    //             triplets.insert([a, b, c]);
    //         }
    //     })
    // });

    //// Pro
    (1..sum / 3).fold(HashSet::new(), |mut acc, a| {
        ((a + 1)..sum / 2).for_each(|b| {
            let c = sum - a - b;
            if a.pow(2) + b.pow(2) == c.pow(2) {
                acc.insert([a, b, c]);
            }
        });
        acc
    })

    // triplets
}
