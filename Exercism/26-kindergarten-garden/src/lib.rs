pub fn plants(diagram: &str, student: &str) -> Vec<&'static str> {
    let index_student = get_students_index(student);
    // let diagram_chars = diagram.chars().into_iter().collect::<Vec<char>>();
    // vec![
    //     name(diagram_chars[2 * index_student]),
    //     name(diagram_chars[2 * index_student + 1]),
    //     name(diagram_chars[2 * index_student + diagram.len() / 2 + 1]), // Hint; there is a line break
    //     name(diagram_chars[2 * index_student + diagram.len() / 2 + 2]),
    // ]
    diagram
        .lines()
        .flat_map(|line| line.chars().skip(2 * index_student).take(2))
        .map(name)
        .collect()
}

fn name(accronym: char) -> &'static str {
    match accronym {
        'V' => "violets",
        'R' => "radishes",
        'C' => "clover",
        'G' => "grass",
        _ => panic!("Invalid accronym"),
    }
}

fn get_students_index(student: &str) -> usize {
    match student {
        "Alice" => 0,
        "Bob" => 1,
        "Charlie" => 2,
        "David" => 3,
        "Eve" => 4,
        "Fred" => 5,
        "Ginny" => 6,
        "Harriet" => 7,
        "Ileana" => 8,
        "Joseph" => 9,
        "Kincaid" => 10,
        "Larry" => 11,
        _ => panic!("Invalid student name"),
    }
}
