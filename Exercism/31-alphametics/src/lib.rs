use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    let mut eqn_parts = input.split("==");
    let lhs = eqn_parts.next().unwrap();
    let rhs = eqn_parts.next().unwrap();

    // Make an array of all the alphabetic characters in both LHS and RHS.
    let mut variables = Vec::new();
    for chars in lhs.chars().chain(rhs.chars()) {
        if !variables.contains(&chars) && chars.is_alphabetic() {
            variables.push(chars);
        }
    }

    println!("{:?}", variables);

    let assignments: HashMap<char, u8> = HashMap::new();
    let mut possible_assignments: Vec<HashMap<char, u8>> = vec![HashMap::new()];
    for var in variables {
        let mut new_possible_assignment = Vec::new();
        for assignment in possible_assignments {
            for digit in 0..10 {
                if !assignment.contains_key(&var) && !assignments.contains_key(&var) {
                    let mut new_assignment = assignment.clone();
                    new_assignment.insert(var, digit);
                    if is_valid_assignment(&new_assignment, lhs, rhs) {
                        new_possible_assignment.push(new_assignment);
                    }
                }
            }
        }
        possible_assignments = new_possible_assignment;
    }

    if possible_assignments.len() == 1 {
        Some(possible_assignments[0].clone())
    } else {
        None
    }
}

fn is_valid_assignment(assignment: &HashMap<char, u8>, left_side: &str, right_side: &str) -> bool {
    let left_value = evaluate_expression(left_side, assignment);
    let right_value = evaluate_expression(right_side, assignment);

    left_value == right_value
        && !has_leading_zero(left_side, assignment)
        && !has_leading_zero(right_side, assignment)
}

fn evaluate_expression(expression: &str, assignment: &HashMap<char, u8>) -> u64 {
    let mut value = 0;
    let mut sign = 1;
    for char in expression.chars() {
        if char.is_alphabetic() {
            value = value * 10 + assignment[&char] as u64;
        } else if char == '+' {
            sign = 1;
        } else if char == ' ' {
            continue;
        } else {
            panic!("Invalid character in expression");
        }
    }
    value * sign
}

fn has_leading_zero(expression: &str, assignment: &HashMap<char, u8>) -> bool {
    let first_char = expression.chars().find(|c| c.is_alphabetic()).unwrap();
    assignment[&first_char] == 0
}
