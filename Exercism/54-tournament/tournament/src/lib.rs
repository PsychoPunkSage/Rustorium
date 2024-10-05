use std::collections::HashMap;

#[derive(Debug)]
struct Performance {
    mp: u32,
    w: u32,
    d: u32,
    l: u32,
    p: u32,
}

pub fn tally(match_results: &str) -> String {
    let mut label = String::from("Team                           | MP |  W |  D |  L |  P");

    if match_results.len() > 0 {
        let mut hashmap = HashMap::new();
        match_results
            .split("\n")
            .collect::<Vec<&str>>()
            .into_iter()
            .for_each(|i| {
                let split = i.split(";").collect::<Vec<&str>>();
                [split[0], split[1]].into_iter().for_each(|team| {
                    hashmap.entry(team).or_insert(Performance {
                        mp: 0,
                        w: 0,
                        d: 0,
                        l: 0,
                        p: 0,
                    });
                    if let Some(perf) = hashmap.get_mut(team) {
                        perf.mp += 1;
                    }
                });

                match split[2] {
                    "win" => {
                        if let Some(perf) = hashmap.get_mut(split[0]) {
                            perf.w += 1;
                            perf.p += 3;
                        }
                        if let Some(perf) = hashmap.get_mut(split[1]) {
                            perf.l += 1;
                        }
                    }
                    "loss" => {
                        if let Some(perf) = hashmap.get_mut(split[0]) {
                            perf.l += 1;
                        }
                        if let Some(perf) = hashmap.get_mut(split[1]) {
                            perf.w += 1;
                            perf.p += 3;
                        }
                    }
                    "draw" => {
                        if let Some(perf) = hashmap.get_mut(split[0]) {
                            perf.d += 1;
                            perf.p += 1;
                        }
                        if let Some(perf) = hashmap.get_mut(split[1]) {
                            perf.d += 1;
                            perf.p += 1;
                        }
                    }
                    _ => unreachable!(),
                }
            });

        //// SORTING
        let mut tally = hashmap
            .iter()
            .map(|(&a, b)| (a, b))
            .collect::<Vec<(&str, &Performance)>>();

        //sort-comparision
        tally.sort_by(|a, b| {
            if a.1.p == b.1.p {
                a.0.cmp(b.0)
            } else {
                b.1.p.cmp(&a.1.p)
            }
        });

        tally.into_iter().for_each(|data| {
            label.push_str(&format!(
                "\n{:<30} | {:>2} | {:>2} | {:>2} | {:>2} | {:>2}",
                data.0, data.1.mp, data.1.w, data.1.d, data.1.l, data.1.p
            ))
        });
    }

    label
}
