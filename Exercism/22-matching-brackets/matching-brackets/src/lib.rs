use std::collections::VecDeque;

pub fn brackets_are_balanced(string: &str) -> bool {
    // let mut stack = VecDeque::new();

    // for c in string.chars() {
    //     match c {
    //         '(' | '{' | '[' => stack.push_back(c),
    //         ')' => {
    //             if stack.pop_back() != Some('(') {
    //                 return false;
    //             }
    //         }
    //         '}' => {
    //             if stack.pop_back() != Some('{') {
    //                 return false;
    //             }
    //         }
    //         ']' => {
    //             if stack.pop_back() != Some('[') {
    //                 return false;
    //             }
    //         }
    //         _ => continue,
    //     }
    // }

    // stack.is_empty()

    let mut copy = string
        .chars()
        .filter(|&c| "[{()}]".contains(c))
        .collect::<String>();
    println!("{}", copy);
    while copy.contains("[]") || copy.contains("{}") || copy.contains("()") {
        copy = copy.replace("[]", "").replace("{}", "").replace("()", "");
    }
    copy.len() == 0
}
