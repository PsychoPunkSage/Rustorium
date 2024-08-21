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

* `Sync`:
    - a type T is `sync` only if &T is Send.
    - i.e. a type whose reference is allowed to shared across the thread, it implements `sync` (Even if the type itself cannot be shared across the thread)
    - IMP: `RC` can't be `sync`; if we pass the ref to some other thread, it (that thread) will call `clone` on it, which we know is not possible as the Clone implementation requires all the access happens on one thread.
    - IMP: On the other hand, `mutex guard` is not `send` BUT it is `Sync`.

    - Type that are not Sync, are those of "interior mutability" in a non-thread-safe environment like `Cell` and `RefCell`
*/

struct Rc<T> {
    inner: *mut Inner<T>,
}

struct Inner<T> {
    count: usize,
    value: T,
}

// Test the functioning of Our Rc
// fn foo<T: Send>(_: T) {}

// fn bar(x: Rc<()>) {
//     foo(x) // Not allowing...
// }

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
    let x = Rc::new(1);
    let y = x.clone();
    std::thread::spawn(move || {
        // Everything moving in must implemet `Send`, here, since RC is not implementing `Send` so. Not OK!!!
        let _ = y;
        drop(y);
    });
    drop(x);
}

/*
If you want to implement -ve trait like `!Send` or `!Sync` then that can't be done on `stable compiler`

SOLN:
    - use `std::marker::PhantomData` as one of the fields of the `type` for which you want to implement -ve trait.
*/
