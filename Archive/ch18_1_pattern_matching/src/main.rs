fn main() {
    // MATCH Expression
    enum Language {
        Hindi,
        English,
        Russian,
        Japanese,
    }

    let language = Language::Hindi;
    match language {
        Language::Hindi => println!("Hindi"),
        Language::English => println!("English"),
        Language::Russian => println!("Russian"),
        Language::Japanese => println!("Japanese"),
        _ => println!("Unknown"), // Not needed here
    }

    // IF-LET Expression
    let auth_stats: Option<&str> = None;
    let is_admin = false;
    let group_id: Result<u8, _> = "49".parse();

    if let Some(auth_status) = auth_stats {
        println!("Current Auth Status{}", auth_status);
    } else if is_admin {
        println!("Auth-status: admin");
    } else if let Ok(group_code) = group_id {
        if group_code > 37 {
            println!("Auth-status: Elevated");
        } else {
            println!("Auth-status: Basic");
        }
    } else {
        println!("Auth-status: GUEST");
    }

    // WHILE-LET Expression
    let mut stack = Vec::new();

    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("{}", top);
    }

    // FOR-LOOP Expression
    let v = vec![1, 2, 3, 4, 5, 6];

    for (i, val) in v.iter().enumerate() {
        println!("{} is at index {}", val, i);
    }

    // LET_STATEMENT Expression
    let x = 5;
    // let PATTERN = EXPRESSION
    let (x, y, z, _) = (1, 2, 4, "garbage");

    // FUNCTION-PARA Expression
    let point = (1, 100);
    print_coordinates(&point);

    // ---------------|
    // PATTERN TYPES  |
    // ---------------|

    // Irrefutable :: always match.. no matter the value
    let x = 3;

    // Refutable :: Won't always match
    let x: Option<String> = None;
    if let Some(string) = x {
        println!("{}", string);
    }

    /*
    Can only accept IRREFUTABLE Pattern:
        - func para
        - let statement
        - for loops
     */
}

fn print_coordinates(/*point*/ &(x, y): &(i32, i32)) {
    println!("Current position: ({}, {})", x, y);
}
