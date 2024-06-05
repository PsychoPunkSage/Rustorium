// Function Pointers
fn add_one(x: i32) -> i32 {
    x + 1
}

fn do_twice<T>(f: T, arg: i32) -> i32
where
    T: Fn(i32) -> i32,
{
    f(arg) + f(arg)
}
// fn do_twice(f: fn(i32) -> i32, arg: i32) -> i32 {
//     f(arg) + f(arg)
// }

/*
There ar 2 function closures:
* Fn    : Captures var and its env immutably.
* FnMut : Captures var and its env mutably.
* FnOnce: Takes ownership of var and its env.
*/

#[derive(Debug)]
enum Status {
    Value(i32),
    Stop,
}

fn main() {
    let ans = do_twice(add_one, 10);
    println!("The Answer is ::> {}", ans);

    // ---------------------------------------------------------------------------------
    let l_o_num = vec![1, 2, 3, 4];
    let l_o_str1: Vec<String> = l_o_num.iter().map(|i| i.to_string()).collect();
    let l_o_str2: Vec<String> = l_o_num
        .iter()
        .map(
            /*Auto implemented of any obj that implements `Display` Trait */
            ToString::to_string,
        )
        .collect();
    println!("List of Strings1: {:?}", l_o_str1);
    println!("List of Strings2: {:?}", l_o_str2);

    // ---------------------------------------------------------------------------------
    let list_of_statuses1: Vec<Status> = (0i32..37).map(Status::Value).collect();
    let list_of_statuses2: Vec<Status> = (0i32..37).map(|i| Status::Value(i)).collect();
    println!("List of Statuses1::> {:?}", list_of_statuses1);
    println!("List of Statuses2::> {:?}", list_of_statuses2);
}

fn return_closures() -> impl Fn(i32) -> i32 {
    |v| v + 1
}

// fn return_closures1(a: i32) -> impl Fn(i32) -> i32 {
//     if a > 0 {
//         move |v| v + a
//     } else {
//         move |v| a - v // no 2 closure even though identical can have same type
//                        // need to return trait object
//     }
// }

fn return_closures1(a: i32) -> Box<dyn Fn(i32) -> i32> {
    // dyn::> returning Trait object of UNKNOWN size
    // Box::> don't know the size of Trait object
    if a > 0 {
        Box::new(move |v| v + a)
    } else {
        Box::new(move |v| a - v)
    }
}
