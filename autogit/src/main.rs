use std::process::{exit, Command};

fn main() {
    let args: Vec<_> = std::env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: {} <Commit_Message> <branchName?>", args[0]);
    };

    // git checkout -b "NAME"
    if args.len() > 2 {
        let checkout_branch = Command::new("git")
            .arg("checkout")
            .arg("-b")
            .arg(args[2].to_string())
            .output()
            .expect("ISSUE:: Branch issue");
        if !checkout_branch.status.success() {
            eprintln!("Error: Failed to make branch <git checkout -b>");
            exit(1);
        }
    }

    // git add .
    let add_command = Command::new("git")
        .arg("add")
        .arg(".")
        .output()
        .expect("ISSUE:: git add");

    if !add_command.status.success() {
        eprintln!("Error: Failed to add files <git add>");
        exit(1);
    }


    // git commit -m "....."
    let commit_command = Command::new("git")
        .arg("commit")
        .arg("-m")
        .arg(args[1].to_string())
        .output()
        .expect("ISSUE:: git commit");
    if !commit_command.status.success() {
        eprintln!("Error: Failed to commit changes <git commit -m>");
        exit(1);
    }

    // git push origin main
    let push_command = Command::new("git")
        .arg("push")
        .arg("origin")
        .arg("main")
        .output()
        .expect("ISSUE:: git push");
    if !push_command.status.success() {
        eprintln!("Error: Failed to push changes <git push origin main");
        exit(1);
    }

    println!("SUCCESS:: Added + Commited + Pushed");
}
