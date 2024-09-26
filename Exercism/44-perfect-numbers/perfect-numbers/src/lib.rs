#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}

pub fn classify(num: u64) -> Option<Classification> {
    if num == 0 {
        return None;
    }

    // let sum = (1..num)
    //     .map(|divisor| if num % divisor == 0 { divisor } else { 0 })
    //     .collect::<Vec<u64>>()
    //     .into_iter()
    //     .sum::<u64>();
    let sum = (1..num).filter(|x| num % x == 0).sum::<u64>();

    match sum {
        sum if sum > num => Some(Classification::Abundant),
        sum if sum == num => Some(Classification::Perfect),
        _ => Some(Classification::Deficient),
    }
}
