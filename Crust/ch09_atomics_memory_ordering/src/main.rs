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
use std::os::unix::thread;
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
        // while self.locked.load(Ordering::Relaxed) != UNLOCKED {} // Waiting for the lock to be released.

        // // PROBLEM: May be another thread run in this brief moment.
        // std::thread::yield_now(); // replicate the problem to show that a thread may execute in between.
        // self.locked.store(LOCKED, Ordering::Relaxed); // Lock before performing any other operation.

        // Soln to above issue.
        // compare_exchange: single operation i.e. Read + Write.
        // BUT it is quite EXPENSIVE ops. -> cause now CPU has to coordinate exclusive references among different threads.
        while self
            .locked
            .compare_exchange(UNLOCKED, LOCKED, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        // we don't want actual updates val, we just wanna know whether val was updated or not.
        {
            // IMP>>> MESI Protocol
            // If any thread fail to take the lock, then load data for it.... i.e. give it READ access. ::>> Popular SPIN Lock implementation.
            while self.locked.load(Ordering::Relaxed) == LOCKED {
                std::thread::yield_now();
            }
            std::thread::yield_now();

            /*
            Why avoid SPIN LOCK:
                - Busy waiting: CPU wastage
                - Reduced Throughput: As CPU is busy and can't perform any other task
                - Priority Inversion: HIGH priority thread may be blocked by LOW priority thread holding SPIN Lock
                - Context Switching Overhead: When multiple threads are spinning on a lock, the operating system might frequently context switch between them, leading to overhead
                    * compare_exchange: impl using the a ""loop"" of LDREX and STREX.
                    * compare_exchange: LDREX and STREX.
            */

            /*
            SPIN Locks V/S Mutexes:
            - SPIN Lock:
                => spinlocks keep the thread busy by repeatedly checking the lock's status until it becomes available.
            - MUTEXES:
                => When a thread fails to acquire a mutex, it's typically blocked and scheduled by the operating system, allowing other threads to run.
            */

            /*
            x86: Compare and Swap Operation.(CAS)
            ARM: LDREX/STREX - Load Exclusive(Takes exclusive of location and memory then loads val)/ Store Exclusive (ONLY if i have exclusive access to the location, ONLY then I will be able to STORE)
            */
        }

        // SAFETY: we hold the lock, therefore we can create a mutable reference.
        let ret = f(unsafe { &mut *self.c.get() }); // we can create mutable ref as no other thread have access to this critical section.
        self.locked.store(UNLOCKED, Ordering::Release); // If used Release: then the next thread that will get the lock will not be able to see the changes we made in previous thread...releases

        /*
        For x86_64 machines
            - this architecture basically guarantees that `acquire-release` semantics for all ops.
            - its by-default; u can't opt out of it.
        For ARM
            - thats not true, if opted for `release` u will get release semantics.
        */
        ret
    }
}

#[test]
fn mutex_test() {
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

#[test]
fn too_relaxed() {
    use std::sync::atomic::AtomicUsize;
    let x: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));
    let y: &'static _ = Box::leak(Box::new(AtomicUsize::new(0)));

    let t1 = spawn(move || {
        let r1 = y.load(Ordering::Relaxed);
        x.store(r1, Ordering::Relaxed);
        r1
    });

    let t2 = spawn(move || {
        let r2 = x.load(Ordering::Relaxed);
        y.store(42, Ordering::Relaxed);
        r2
    });

    // When above threads execute we have Modification Order foreach var.
    // MO(x): 0 -> 42
    // MO(y): 0 -> 42
    // Under `Ordering::Relaxed` (x) is allowed to see any value to be stored in x. ~~ TIME TRAVEL

    let r1 = t1.join().unwrap();
    let r2 = t2.join().unwrap();

    // r1 == r2 == 42
}

/*
ORDERING:
    * Ordering::Relaxed
        - there is no guarantees about what values thread can read from something another thread wrote.
        - doesn't establish any order between any thread and code to be executed within the thread.
        - i.e CPU Compiler can rejigger the lines of code to get significant performance gain....

    * Ordering::Release
        - only applicable for ops that can perform a store.
        - all previous write becomes visible to all threads that perform an Acquire (or stronger) load of this value.
        - all previous operation become ordered before any load of this value(data on which I'm applying ordering) with an Acquire.
        - nothing can be reordered after a Release-Store.

    * Ordering::Acquire
        - a LOAD ops is performed with this memory order.
        - no read or writes in the current thread can be reordered before this LOAD.
        - all write in other threads that release the same Atomic Variable are visible in the current thread.

    * Ordering::AcqRel
        - usually passed in the ops that usually read or write. e.g. compare_exchange
        - do the load with Acquire semantics and Store with release semantics.
*/

fn main() {}
