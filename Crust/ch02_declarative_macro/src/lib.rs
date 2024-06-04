/*
Macros are not allowed to access things outside of their own scope.

ident:: all the expr and vars inside `macro` exist in different spaces/world. So we cannot access/impl those directly. So we use `ident` for this
*/

#[macro_export]
macro_rules! ppsvec {
    // ($arg1:ty, $arg2:expr, $arg3:path) => {};
    // ($arg1:ty => $arg2:ident) => {
    //     type $arg2 = $arg1;
    // };

    // ($arg1:ty as $arg2:ident) => {
    //     type $arg2 = $arg1;
    // };

    // ($x:ident) => {
    //     $x += 1;
    // };

    () => {
        Vec::new()
    };

    ($ele:expr) => {{
        /*
        Need {{}} : Block
            - outer {}: it is what macro_rules! require us to do...
                        i.e. this is the chunk of thing something expands to... cause complex macros can expand to multiple modules
            - inner {}: it signifies expressions... if we don't use it we will get a sequence of expressions which doesn't
                        qualifies as valid expression.
        */
        let mut vs = Vec::new();
        vs.push($ele);
        vs
    }};

    ($ele1:expr, $ele2:expr) => {{
        /*
        Need {{}} : Block
            - outer {}: it is what macro_rules! require us to do...
                        i.e. this is the chunk of thing something expands to... cause complex macros can expand to multiple modules
            - inner {}: it signifies expressions... if we don't use it we will get a sequence of expressions which doesn't
                        qualifies as valid expression.
        */
        let mut vs = Vec::new();
        vs.push($ele1);
        vs.push($ele2);
        vs
    }};

    // `,` ::> We can use any single rust token in place of `,` <Given the compatibility>
    // `?` ::> either the sequence is present or it is not present.
    // ($($element:expr),+ $(,)?) => {
    //     {
    //         let mut vs = Vec::new();
    //         $(vs.push($element);)+ // + ::> to show repetition
    //         vs
    //     }
    // };

    ($element:expr; $count:expr) => {
        {
            let count = $count;
            let mut vs = Vec::with_capacity(count);
            // let x = $element;
            // for _ in 0..count {
            //     // [DUMB]
            //     // Cause it checks allocation for each and every push.
            //     vs.push(x.clone());
            // }
            // vs.extend(::std::iter::repeat($element).take(count)); // Wayyy... efficient
            vs.resize(count, $element); // Even better... No need to do bound check
            vs
        }
    };

    ($($element:expr),* $(,)?) => {
        {
            // [CHECK] that `count` is constant..
            const C: usize = $crate::count![@COUNT; $($element),*];
            #[allow(unused_mut)]
            let mut vs = Vec::with_capacity(C);
            $(vs.push($element);)*
            vs
        }
    };
}

#[macro_export]
#[doc(hidden)]
macro_rules! count {
    (@COUNT; $($element:expr),*) => {
        [$($crate::count![@SUBST; $element]),*].len()

    };
    (@SUBST; $_element:expr) => {
        ()
    };
}

trait MaxValue {
    fn max_value() -> Self;
}

macro_rules! max {
    ($t:ty) => {
        impl $crate::MaxValue for $t {
            fn max_value() -> Self {
                <$t>::MAX
            }
        }
    };
}

// Shorthand to implement common pattern
max!(i32);
max!(u32);
max!(u64);
max!(i64);

// fn foo() {
//     let mut x = 10;
//     ppsvec! {x}
//     println!("{}", x);
// }

// ppsvec! {
//     u32 => alsou32
// }

// ppsvec!(u32 as notalsou32);

// ppsvec! [
//     u32 => notalsoau32
// ];

#[cfg(test)]
mod tests {

    #[test]
    fn empty_vec() {
        let x: Vec<u32> = ppsvec![];
        assert!(x.is_empty());
    }

    #[test]
    fn single_ele_vec() {
        let x: Vec<u32> = ppsvec![23];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 1);
        assert_eq!(x[0], 23);
    }

    #[test]
    fn double_ele_vec() {
        let x: Vec<u32> = ppsvec![23, 32];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 2);
        assert_eq!(x[0], 23);
        assert_eq!(x[1], 32);
    }

    #[test]
    fn multiple_ele_vec() {
        let x: Vec<u32> = ppsvec![
            23, 32, 34, 103, 67, 293, 2949, 402, 3924, 7, 89, 20, 9403, 944, 103, 49, 95, 4, 34,
        ];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 19);
    }

    #[test]
    fn shorthand_ele_vec() {
        let x = ppsvec![12; 12];
        assert!(!x.is_empty());
        assert_eq!(x.len(), 12);
    }
}
