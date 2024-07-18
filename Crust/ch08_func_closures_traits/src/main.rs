fn main() {
    println!("Hello, world!");
    let x = bar; // function item
    let x1 = bar1::<i32>; // generic function item
                          // x = bar1::<u32>(); cannot reassign a with different generic.
    let x2 = bar2; // function item

    println!(
        "SIZE: x - {} || x1 - {}",
        std::mem::size_of_val(&x),
        std::mem::size_of_val(&x)
    );

    /*
    Funtion Item:
        - "Zero sized" value that is only carried around at compile time the reference a unique function.
        - Its just an `Identifier`.
        - it uniquely identifies a particular instance of a function.

    Function Pointer:
        - pointer to a function with a given signature.
    */
    bazz(bar1::<i32>);
    bazz(bar1::<u32>);

    // `Function Item` & `Function Pointer` are different from one another. BUT `Function Item(s)`  are coersible inside the `Function Pointer`.
    // `Function Item` & `Function Pointer` DON'T have any STATE.
    //                                      DON'T have any LIFETIME associated with them.
    //                                      DON'T reference any memory outside of themselves.
    //                                      IMPLEMENTS all the 3 traits i.e. Fn, FnMut, FnOnce. <REASON: doesn't capture environment> OOR since it implements `Fn` we can say it can also implement `FnMut` and `FnOnce`
    // Demonstrated above.

    quox(bar1::<i32>);
    quox1(&mut bar1::<i32>);
    quox2(&bar1::<i32>);

    /*
    Closure:
        - Non-capturing closures
            * coersible to Function pointers.
        - Capturing closures
            * NOT coersible to Function pointers. Because we need to know the state of each element in the closure.
    */

    let mut f = |x: u32| return x;
    /*
    WHAT compiler generates from `closure`

    struct f_closure <'scope> {
        _any_captures_var: &'scope <type>,
    }

    impl<'scope> Fn() for f_closure<'scope> {
        fn call(&self) {
            // copy everything from closure def....
        }
    }
    */
    bazz(f);
    quox(f);
    quox1(&mut f);
    quox2(&f);
}

fn bar() {}
fn bar1<T>(_: u32) -> u32 {
    0
}
async fn bar2() {}

fn bazz(f: fn(u32) -> u32) {
    // Now here the `function item` is properly converted to `fn pointer`.
    println!("bazz: {}", std::mem::size_of_val(&f));
}

/*
    * Fn:
        - Takes `shared reference` of itself i.e. &self
        - Can be called multiple times AND it can be called multiple times at the SAME Time.
    * FnMut:
        - Takes `exclusive reference` of itself i.e. &mut self
        - Can be called multiple times, but each time it can only be called ONCE.
        - Ex.: It doesn't work with `ARC` or in multi-thread environments.
    * FnOnce:
        - Takes `owned reference` of itself i.e. self
        - can be called `SINGLE` time.

    >> Anything that implements `Fn`    also implements <FnMut> and <FnOnce>
                                `FnMut` also implements <FnOnce> But not <Fn>
*/

fn quox<T>(f: T)
where
    T: FnOnce(u32) -> u32,
{
    println!("quox: {}", std::mem::size_of_val(&f))
}

fn quox1<T>(f: &mut T)
where
    T: FnOnce(u32) -> u32,
{
    println!("quox1: {}", std::mem::size_of_val(&f))
}

fn quox2<T>(f: &T)
where
    T: Fn(u32) -> u32,
{
    println!("quox2: {}", std::mem::size_of_val(&f))
}
