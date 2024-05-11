fn main() {
    /*
    ------------------Ownership RULES ------------------
    1. Each value has variable that is called its own `Owner`.
    2. There can only be one owner at a time.
    3. When the owner goes out of scope, the value will be dropped.
     */
    let x = 5;
    let y = x; // COPY (not move) - only for simple DS.

    let s1 = String::from("AP is here");
    let s2 = s1.clone();

    println!("s1 = {}, s2 = {}", s1, s2);
}
