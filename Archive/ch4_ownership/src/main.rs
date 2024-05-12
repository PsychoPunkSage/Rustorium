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

    // Change the value of Reference - USE mutable reference.
    //                               - ONLY 1 mutable reference in SCOPE.
    //                               - Can't have mutable reference if mutable reference EXISTS. <But we can define Immutable Ref if Mutable_ref is out off scope>

    let mut s1 = String::from("Hello");
    change(&mut s1);
    println!("s1 = {}", s1);

    /*
    ------------------Reference RULES ------------------
    1. At one time::> 1 mutable_ref OOOR inf immutable_ref
    2. Reference must always be VALID.
     */

    let mut s = String::from("AP is here");
    let ap = &s[..2];
    let is = &s[3..5];
    let here = &s[6..];
    let word = first_word(&s);
    let word_modified = first_word_modified(&s);
    println!("ap = {}, is = {}, here = {}", ap, is, here);
    s.clear();
    println!("word = {}", word); // the will return `2` even though `s` DNE now --> Full of errors
                                 // println!("word_modified = {}", word_modified); /// GIVES ERROR ::> Doesn't allow to borrow.... as "s" -> getting cleared.

    // ARRAYS:
    let a = [1, 2, 3, 4, 5, 6];
    let slice_a = &a[..3];
}

fn change(some_str: &mut String) {
    some_str.push_str("APP");
}

// SLICES - reference a part of collection instead of whole collection.
fn first_word(s: &String) -> usize {
    let b = s.as_bytes(); // convert to array of bytes

    for (i, &item) in b.iter().enumerate() {
        // Check for spaces
        if item == b' ' {
            return i; // return value is not tied to string itself
        }
    }

    s.len()
}

fn first_word_modified(s: &String) -> &str {
    let b = s.as_bytes(); // convert to array of bytes

    for (i, &item) in b.iter().enumerate() {
        // Check for spaces
        if item == b' ' {
            return &s[..i]; // return value is not tied to string itself
        }
    }

    &s[..]
}
