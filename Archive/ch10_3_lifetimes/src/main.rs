fn main() {
    // Ex1
    let string1 = String::from("AP ");
    let string2 = String::from("is here");
    println!("result::> {}", longest(string1.as_str(), string2.as_str()));

    // Ex2
    let str1 = String::from("Test");
    let result;
    {
        let str2 = String::from("Cases");
        result = longest(str1.as_str(), &str2);
        // lifetime(result) == lifetime(str2)  ||| lifetime(str2) < lifetime(str1)
    }
    println!("Longest string: {}", result);
}

/*
Lifetime Annotations:
-> &i32 ::>  a reference
-> &'a i32 ::> a reference with an explicit lifetime
-> &'a mut i32 ::> a mutable reference with an explicit lifetime
*/

// Generic Lifetime annotation
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    /*
    * lifetime(return) MUST always be tied to lifetime(parameters)....
    Cause we can't return reference of something generated inside the fn <as once fn is over; the var will go out of scope>

    This does NOT mean that lifetime(x) == lifetime(y)
    It just establish a relationship between x, y and return type
    => lifetime(return) == min(lifetime(x), lifetime(y))
     */
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

fn dangling_reference() {
    let mut r;

    {
        let x = 5;
        r = &x; // x will get out of scope.
    }

    r = &10;
    println!("r: {}", r); // r -> Dangling Reference
}
