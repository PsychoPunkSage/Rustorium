#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq + std::fmt::Debug>(first_list: &[T], second_list: &[T]) -> Comparison {
    let str1 = first_list
        .iter()
        .map(|x| format!("{:?}:", x))
        .collect::<String>();

    let str2 = second_list
        .iter()
        .map(|x| format!("{:?}:", x))
        .collect::<String>();

    if str1.len() == str2.len() {
        if str1 == str2 {
            return Comparison::Equal;
        } else {
            return Comparison::Unequal;
        }
    }

    if str1.len() >= str2.len() {
        if str1.contains(&str2) {
            return Comparison::Superlist;
        }
        return Comparison::Unequal;
    }

    if str1.len() <= str2.len() {
        if str2.contains(&str1) {
            return Comparison::Sublist;
        }
        return Comparison::Unequal;
    }

    Comparison::Unequal
}
