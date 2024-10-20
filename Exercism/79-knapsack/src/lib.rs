#[derive(Debug)]
pub struct Item {
    pub weight: u32,
    pub value: u32,
}

pub fn maximum_value(max_weight: u32, items: &[Item]) -> u32 {
    if max_weight == 0 || items.len() == 0 {
        return 0;
    }

    if (max_weight as i32 - items[0].weight as i32) < 0 {
        return maximum_value(max_weight, &items[1..]);
    }

    return std::cmp::max(
        items[0].value + maximum_value(max_weight - items[0].weight, &items[1..]),
        maximum_value(max_weight, &items[1..]),
    );
}
