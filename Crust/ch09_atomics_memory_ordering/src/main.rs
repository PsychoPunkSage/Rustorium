/*
ATOMICUSIZE
    > designed for `thread-safe` operations. It's a wrapper around the underlying `usize` type, providing atomic operations that guarantee `data consistency` even in `concurrent` environments
        - Thread Safety:   AtomicUsize is thread-safe, while usize is not.
        - Operations:      AtomicUsize provides atomic operations like fetch_add, compare_and_swap, and load, which are not available for usize.
        - Memory Ordering: AtomicUsize offers control over memory ordering through Ordering parameters, allowing you to specify the level of synchronization required.
        - Alignment:       AtomicUsize has stricter alignment requirements than usize to ensure atomic operations work correctly.

    > leverages low-level CPU instructions to perform atomic operations. These instructions are hardware-guaranteed to be indivisible, preventing race conditions.

ORDERING:
    > tells compiler which set of guarantees you expect for this particular memory access wrt things that might be happening in other threads at the same time.

>> Atomic provides lots of unique fn to deal with data. Like `store`, `load`, `compare_and_swap`, `swap` etc.
    * When we load and store data separately then there is a chance that a random THREAD can executed data change in between out `load & store` calls.
    * to avoid this ATOMIC provides SINGLE step fn that will `load and store` data in one step disallowing other to execute in between.

MUTEX V/S ATOMIC
    Atomic:
        * Doesn't use locking.
        * Here multiple threads can operate on this value at the same time in some reasonably well-defined way.
        * All atomic ops are "lock-free" but not necessarily "wait-fre".
    Mutex:
        * One thread gets to access the value at a time. And other have to wait until the existing tread releases the lock.
        * Mutex guards larger section of code.
*/

use std::cell::UnsafeCell;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::spawn;

const LOCKED: bool = true;
const UNLOCKED: bool = false;

/*
Mutex 2 parameters:
    - Boolean:    marks whether the Lock is UP or not.
    - UnsafeCell: id we have a lock we can give out Mutable reference of the Inner-type.
*/
pub struct Mutex<T> {
    locked: AtomicBool,
    c: UnsafeCell<T>,
}

unsafe impl<T> Sync for Mutex<T> where T: Send {}

impl<T> Mutex<T> {
    pub fn new(t: T) -> Self {
        return Mutex {
            locked: AtomicBool::new(UNLOCKED),
            c: UnsafeCell::new(t),
        };
    }
    pub fn with_lock<R>(&self, f: impl FnOnce(&mut T) -> R) -> R {
        // Now we gonna lock the Mutex
        while self.locked.load(Ordering::Relaxed) != UNLOCKED {} // Waiting for the lock to be released.

        // PROBLEM: May be another thread run in this brief moment.
        std::thread::yield_now(); // replicate the problem to show that a thread may execute in between.

        self.locked.store(LOCKED, Ordering::Relaxed); // Lock before performing any other operation.

        // SAFETY: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.c.get() }); // we can create mutable ref as no other thread have access to this critical section.
        self.locked.store(UNLOCKED, Ordering::Relaxed);
        ret
    }
}

fn main() {
    let l: &'static _ = Box::leak(Box::new(Mutex::new(0))); // Cause we need static ref of `Box::Leak`
    let handles: Vec<_> = (0..10)
        .map(|_| {
            spawn(move || {
                for _ in 0..100 {
                    l.with_lock(|v| {
                        *v += 1;
                    })
                }
            })
        })
        .collect();

    for handle in handles {
        handle.join().unwrap(); // wait for the thread to finish and then get the value.
    }

    assert_eq!(l.with_lock(|v| *v), 10 * 100)
}
