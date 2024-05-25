use std::slice;

//////////////////////////////////////
// FFI (Foreign Function Interface) //
//////////////////////////////////////
/*
When we need to interact with foreign functions written in different languages.
 */

extern "C" {
    fn abs(input: i32) -> i32;
}

// Allowing other languages to call Rust functions
#[no_mangle] // tells Rust compiler not to change the name of the function.
pub extern "C" fn call_from_c() {
    println!("Just called a RUST function from C!!");
}

/*
static vars ~ const vars
Difference:
static::> always points to fixed address in memory....
      ::> can be mutable... accessing mutable static variables is `unsafe`
const::> allowed to duplicate itself whenever used.
*/
static HELLO_AP: &str = "hello AP"; // Must have a static lifetime.
static mut COUNTER: u32 = 0;

fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

// <4.> Unsafe trait
unsafe trait Foo {
    // Trait is unsafe if at-least one of its function is unsafe.
}

unsafe impl Foo for i64 {}

fn main() {
    /*
    UNSAFE Rust GIVES us 5 abilities:
    - Dereference a raw pointer
    - Call an unsafe function or method
    - Access or modify a mutable static variable
    - Implement an unsafe trait
    - Access fields of an `UnsafeCell<T>` (interior mutability) || Or union

    Doesn't turn off:
    - Rust Borrows checker
    - Rust Safety checks
     */

    // RAW-Pointers
    let mut num = 19;
    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;
    /*
    *** Raw-pointer is allowed to SKIP `Borrow` rules
    ** Allowed multiple immutable/mutable raw pointers at same time.
    ** Not Guaranteed to point to valid memory.
    ** Allowed to be NULL.
    ** Doesn't implement auto cleanup

    immutable RAW: pointer can't be directly reassigned after being dereferenced.
     */

    let rand_mem_add = 0x012345usize;
    let r3 = rand_mem_add as *const i32;

    // Unsafe Fn/methods
    unsafe fn dangerous() {}

    unsafe {
        // need this block to `dereference` Raw pointers.
        println!("r1 is: {}", *r1);
        println!("r2 is: {}", *r2);
        dangerous(); // `unsafe`: Only allowed to call inside another `unsafe fn` or `Unsafe` block
    }

    // Unsafe code inside Safe-block
    let mut v = vec![1, 2, 3, 4, 5, 6];
    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);
    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // FFI:
    unsafe {
        println!("Absolute value of -3 according to C: {}", abs(-3));
    }

    // 3. Static Vars
    println!("Name is {}", HELLO_AP);
    add_to_count(97);
    unsafe {
        // require unsafe block to access it.
        println!("Static Count is {}", COUNTER);
    }
}

// fn safe_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
//     assert!(mid < slice.len());
//     (&mut slice[..mid], &mut slice[mid..]) // Error: Borrow check....
// }

fn unsafe_split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    assert!(mid < slice.len());
    let mut_raw_ptr = slice.as_mut_ptr();

    unsafe {
        (
            slice::from_raw_parts_mut(mut_raw_ptr, mid),
            slice::from_raw_parts_mut(mut_raw_ptr.add(mid), slice.len() - mid),
        )
    }
}
