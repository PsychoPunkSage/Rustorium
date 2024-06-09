use std::cell::UnsafeCell;

#[warn(dead_code)]
pub struct Cell<T> {
    value: UnsafeCell<T>,
}

/*
The ONLY way in Rust to go from a shared reference to an exclusive reference is with `UnsafeCell`.
    - We are not allowed to cast a shared reference to an exclusive reference...
Reason:: How Rust compiler optimize code.
*/

// impleied by `UnsafeCell`
// impl<T> !Sync for Cell<T> {}

// unsafe impl<T> Sync for Cell<T> {}

impl<T> Cell<T> {
    pub fn new(value: T) -> Self {
        Cell {
            value: UnsafeCell::new(value),
        }
    }

    pub fn set(&self, value: T) {
        // But what will happen if 2 different `threads` changes the value at same time... there is nothing to prevent it...
        // So we must implement <!Sync> in Cell<T>
        /// SAFETY: we know no-one else is concurrently mutating the self.value (because of !Sync)
        /// SAFETY: we know we're not invalidating any reference, because we never give anything out <via. any reference>.
        unsafe {
            *self.value.get() = value
        };
    }

    pub fn get(&self) -> T
    where
        T: Copy,
    {
        /// SAFETY: we know no-one else is modifying this value, since only this thread can mutate (cause of !Sync) but @this_moment it is implementing `get` so, `set` is not being called.
        unsafe {
            *self.value.get()
        }
    }
}

#[cfg(test)]
mod test {
    use super::Cell;

    // #[test]
    // // #[should_panic]
    // fn sync_test_mutli_thread() {
    //     use std::sync::Arc;
    //     use std::thread;
    //     let c = Arc::new(Cell::new(0));

    //     let c1 = Arc::clone(&c);
    //     let jh1 = thread::spawn(move || {
    //         for _ in 0..100000 {
    //             let x = c1.get();
    //             c1.set(x + 1);
    //         }
    //     });

    //     let c2 = Arc::clone(&c);
    //     let jh2 = thread::spawn(move || {
    //         for _ in 0..100000 {
    //             let x = c2.get();
    //             c2.set(x + 1);
    //         }
    //     });
    //     // The threads will race to write its walue
    //     // Due to this the value `read` by a thread may not be the same as when it writes it....
    //     // SOOOO we need to make sure `Sync` is not implemented by `Cell`
    //     jh1.join().unwrap();
    //     jh2.join().unwrap();
    //     eprintln!("Total sum: {}/200000", c.get());
    //     assert!(c.get() < 200000);
    // }

    // #[test]
    // fn single_thread_bad() {
    //     let x = Cell::new(String::from("AP"));
    //     let first = x.get();
    //     eprintln!("First::> {}", first);
    //     x.set(String::from("changed"));
    //     eprintln!("Second::> {}", first);
    // }
}
