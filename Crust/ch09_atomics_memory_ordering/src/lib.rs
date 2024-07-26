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
    Mutex:
        * One thread gets to access the value at a time. And other have to wait until the existing tread releases the lock.
        * Mutex guards larger section of code.
*/
