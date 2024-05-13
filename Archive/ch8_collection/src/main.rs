use std::collections::HashMap;
use unicode_segmentation::UnicodeSegmentation;

fn main() {
    let a = [1, 2, 3, 4, 5, 6];
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    v.push(3);

    // Another representation
    let v2 = vec![1, 2, 3, 4];
    println!("{:?}", v2);
    println!("{}", &v2[2]);

    // Safer access of elements in Vector ::> ise "".get(index)""
    match v.get(3) {
        Some(ele) => println!("4th element {}", ele),
        None => println!("No third element"),
    }

    // Parsing a vector
    for i in a {
        println!("{}", i);
    }

    for ji in &mut a.iter() {
        println!("{}", *ji + 50);
    }

    for j in a.iter() {
        println!("{}", *j);
    }

    for (k, l) in a.iter().enumerate() {
        // k -> index || l -> element reference
        println!("{} {}", k, l);
    }

    // ENUM in Vectors
    #[derive(Debug)]
    enum Spreadsheet {
        Int(i32),
        Float(f64),
        Text(String),
    }

    let row = vec![
        Spreadsheet::Int(1),
        Spreadsheet::Float(12.45),
        Spreadsheet::Text(String::from("Mock")),
    ];

    println!("{:#?}", row);

    // Check
    match &row[1] {
        Spreadsheet::Int(_) => println!("Its an integer"),
        _ => println!("Not an integer"),
    }

    // String Treatment::> Very complex for Low-level programming
    /*
    Rust Uses UTF-8 encoding ::> Any character is represented in 1/2/3/4 bytes <Unlike ASCII for which 1 Char = 1 Byte>
     */

    let mut s = String::from("foo");
    s.push_str("bar"); // taking in `string reference`
    s.push('!');

    let st1 = String::from("Hello ");
    let st2 = String::from("AP");
    let st4 = format!("{}{}", st1, st2);
    let st3 = st1 + &st2; // Owner of `st1`
    println!("{} || {}", st3, st4);

    // Unicode Parsing of `String` literals
    /*
    “नमस्ते”
    -> u8 = [224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164, 224, 165, 135]
    -> Scalar Values = ['न', 'म', 'स', '्', 'त', 'े']
    -> Graphene clusters = ["न", "म", "स्", "ते"] // We can't do this type of parsing in RUST. Need to use `unicode-segmentation` crate.
    */

    let hello = String::from("नमस्ते");
    // for u8
    for b in "नमस्ते".bytes() {
        println!("{}", b)
    }

    // for Scalar
    for c in "नमस्ते".chars() {
        println!("{}", c)
    }

    // for Graphemes
    for g in "नमस्ते".graphemes(false) {
        println!("{}", g)
    }

    // HASHMAP
    let t_b = String::from("blue");
    let t_y = String::from("yellow");

    let mut scores = HashMap::new();

    scores.insert(t_b, 10); // Ownership of blue String moved to Hashmap
    scores.insert(t_y, 23);

    scores.entry(String::from("green")).or_insert(30); // If `green` DNE then insert it with default value of `30` ELSE do nothing.

    // retrieve values
    println!("{:?}", scores);
    println!("Score of Blue Team: {}", scores.get("blue").unwrap()); // Used unwrap to extract i32 from `Options`.

    let text = "No one can beat GOKU. No one";
    let mut n_hash = HashMap::new();

    for word in text.split_whitespace() {
        let w_count = n_hash.entry(word).or_insert(0);
        *w_count += 1;
    }
    println!("map {:?}", n_hash);
}
