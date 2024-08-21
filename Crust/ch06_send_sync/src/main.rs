/*
RUST have 2 traits <Send> and <Sync> which is used to Define `THREAD SAFETY` in the language.
    - Define Tread Safety at type level....

Similarities: <Send/Sync>
    - Both of them are MARKER TRAITS.
        - These are the Traits with NO METHODS.
    - `Send/Sync` also implement `Auto` Trait.
        * This means that the compiler will Automatically implement this trait for you IF all the members of a type are themselves that same Trait
        * Ex.: For struct and Enum etc. will implement `Send/Sync` if all the types inside of it also implement `Send/Sync`.
    - ALL `Auto` traits are `Marker` Traits But all `Marker` traits are not `Auto` Traits.

Differences:
* `Send`:
    - Its OK to pass the value to differnet thread.
    - i.e. giveaway the ownership of this thing to some other thread. So, that thread can do whatever it wants to do with that `value`.
    - MOST types are `Send` execpt `RC` & `Mutex Guard`
        * For `Std. Mutex` that are backed by OS implementation. They are not Send cause there are requirements on Certain OS that the `thread` that gets the LOCK should be the one that releases the LOCK.
        * IMPORTANT: `Mutex` and `RW Lock` are SEND || But the `GUARD` is NOT SEND.
        * Ex.: Suppose you are Dropping a Val that hase to Access the thread local of the thread that created it.... Now suppose THREAD-A create the Val, now, it is sent to THREAD-B and THREAD-B tries to drop the Val. So the Val will try to access the `thread Local` of THREAD-B (instead of THREAD-A) which will violate internal Invariants.
*/

struct Rc<T> {
    inner: *mut Inner<T>,
}

struct Inner<T> {
    count: usize,
    value: T,
}

impl<T> Rc<T> {
    fn new(value: T) -> Self {
        Rc {
            inner: Box::into_raw(Box::new(Inner { count: 1, value })),
        }
    }
}

impl<T> Clone for Rc<T> {
    fn clone(&self) -> Self {
        unsafe {
            (&mut *self.inner).count += 1;
        }
        Self { inner: self.inner }
    }
}

impl<T> Drop for Rc<T> {
    fn drop(&mut self) {
        let cnt = unsafe { (*self.inner).count };
        if cnt == 1 {
            let _ = unsafe { Box::from_raw(self.inner) };
        } else {
            unsafe {
                (*self.inner).count -= 1;
            }
        }
    }
}

impl<T> std::ops::Deref for Rc<T> {
    type Target = T;
    fn deref(&self) -> &Self::Target {
        &unsafe { &*self.inner }.value
    }
}

fn main() {
    println!("Hello, world!");
}
