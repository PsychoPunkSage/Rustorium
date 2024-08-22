#![allow(unused)]

use std::ops::{Deref, DerefMut};
fn main() {
    pub mod libc {
        pub use ::std::os::raw::{c_int, c_void};
        #[allow(non_camel_case_types)]
        pub type size_t = usize;
        extern "C" {
            pub fn posix_memalign(memptr: *mut *mut c_void, align: size_t, size: size_t) -> c_int;
        }
    }
    use std::{
        cmp::max,
        mem::{align_of, size_of},
        ptr,
    };

    /*
        Carton<T>
            * manages memory allocation and storage for any type T using manual memory management (similar to how smart pointers work in Rust)
    */
    struct Carton<T>(ptr::NonNull<T>);

    impl<T> Carton<T> {
        pub fn new(value: T) -> Self {
            // Allocate enough memory on the heap to store one T.
            assert_ne!(
                size_of::<T>(),
                0,
                "Zero-sized types are out of the scope of this example"
            );
            let mut memptr: *mut T = ptr::null_mut();
            unsafe {
                let ret = libc::posix_memalign(
                    // C library function that allocates memory with specific alignment requirements:
                    (&mut memptr as *mut *mut T).cast(),
                    max(align_of::<T>(), size_of::<usize>()),
                    size_of::<T>(),
                );
                assert_eq!(ret, 0, "Failed to allocate or invalid alignment");
            };

            // NonNull is just a wrapper that enforces that the pointer isn't null.
            let ptr = {
                // Safety: memptr is dereferenceable because we created it from a
                // reference and have exclusive access.
                ptr::NonNull::new(memptr).expect("Guaranteed non-null if posix_memalign returns 0")
            };

            // Move value from the stack to the location we allocated on the heap.
            unsafe {
                // Safety: If non-null, posix_memalign gives us a ptr that is valid
                // for writes and properly aligned.
                ptr.as_ptr().write(value);
                ////// This moves the value `value: T` from the stack to the heap by writing it to the memory location pointed to by `ptr`.
            }

            Self(ptr)
        }
    }

    impl<T> Deref for Carton<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            unsafe {
                // Safety: The pointer is aligned, initialized, and dereferenceable
                //   by the logic in [`Self::new`]. We require readers to borrow the
                //   Carton, and the lifetime of the return value is elided to the
                //   lifetime of the input. This means the borrow checker will
                //   enforce that no one can mutate the contents of the Carton until
                //   the reference returned is dropped.
                self.0.as_ref()
            }
        }
    }

    impl<T> DerefMut for Carton<T> {
        fn deref_mut(&mut self) -> &mut Self::Target {
            unsafe {
                // Safety: The pointer is aligned, initialized, and dereferenceable
                //   by the logic in [`Self::new`]. We require writers to mutably
                //   borrow the Carton, and the lifetime of the return value is
                //   elided to the lifetime of the input. This means the borrow
                //   checker will enforce that no one else can access the contents
                //   of the Carton until the mutable reference returned is dropped.
                self.0.as_mut()
            }
        }
    }

    let mut a = 12;
}

/*
SEND:
    - Something can safely be Send unless it shares mutable state with something else without enforcing exclusive access to it.
    - Since `Carton` is not giving `mut reference`, so we're good
    - OR, Carton holds a unique pointer to some data on the heap. This means that each Carton instance has its own pointer, and no other Carton or part of the program can access that same pointer unless it's explicitly given. ecause the pointer is unique, there’s no risk of two threads accidentally modifying the same data at the same time.

SYNC:
    - In the case of Carton, you need a mutable reference (&mut Carton) to write to the data. Rust's borrow checker ensures that a mutable reference is exclusive—meaning only one thread can have that reference at a time.
    -
*/
