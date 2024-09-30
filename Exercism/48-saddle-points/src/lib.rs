pub fn find_saddle_points(input: &[Vec<u64>]) -> Vec<(usize, usize)> {
    let mut ans = Vec::new();

    for (i, v) in input.iter().enumerate() {
        if let Some(&max) = v.iter().max() {
            for (max_index, &ele) in v.iter().enumerate() {
                if ele == max {
                    let is_the_point = input.iter().all(|r| r[max_index] >= max);
                    if is_the_point {
                        ans.push((i, max_index));
                    }
                }
            }
        }
    }

    return ans;
}
