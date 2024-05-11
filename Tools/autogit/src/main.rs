use std::{process::{exit, Command}, io::{self, Write}};

fn prompt(prompt: &str) -> String {
    print!("{} ", prompt);
    io::stdout().flush().unwrap();

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();

    input.trim().to_string()
}


fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <Commit_Message> <branchToBePushedOn!> <branchName?>", args[0]);
    };

    
    // git checkout -b "NAME"
    if args.len() > 3 {
        let checkout_branch = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(args[3].to_string())
            .output()
            .expect("ISSUE:: Branch issue");
        if !checkout_branch.status.success() {
            eprintln!("Error: Failed to make branch <git checkout -b '{}'>", args[3].to_string());
            exit(1);
        } else {
            println!("<<git checkout -b {}>> :: Successful", args[3].to_string());
        }
    }

    // git add .
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("ISSUE:: git add");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files <git add .>");
        exit(1);
    } else {
        println!("<<git add .>> :: Successful");
    }


    // git commit -m "....."
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(args[1].to_string())
        .output()
        .expect("ISSUE:: git commit");
    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes <git commit -m '{}'>", args[1].to_string());
        exit(1);
    } else {
        println!("<<git commit -m {}>> :: Successful", args[1].to_string());
    }


    // git push origin main
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg(args[2].to_string())
        .output()
        .expect("ISSUE:: git push");
    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes = <git push origin {}>", args[2].to_string());
        exit(1);
    } else {
        println!("<<git push origin {}>> :: Successful", args[2].to_string());
    }

    println!("SUCCESS:: Added + Commited + Pushed");
}
