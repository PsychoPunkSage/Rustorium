/*
THREAD_SANITIZER_ALGORITHM:
    - runs code in an instrumented way where every load and store we do on gets tracked (like what and when it modifies)
    - Tries to detect unsyc operation in a single memory location.
    - Once we run code, it will detect if that execution of the code did anything bad.

LOOM:
    - OS Rust based project.
    - It takes concurrent program, instrument it, it will run it multiple times, And everytime we do a load through one of the loom DS, it will feed us back one of the possible Legal vals.
    - When stored; it will keep track of all the values that have been stored so that on other loads, it will expose the execution to one of the possible loads.
*/
/*
Library:> std::sync::atomic::fence

COMPILER FENCE:
    - way to ensure that the compiler won't move a given loader store.
    - SIMPLER: Compiler is not allowed to move an operation under the fence to above a fence / above a fence to below a fence within that thread execution.
    - ONLY FOR COMPILERS, CPU can still execute things out of order.
    - RARELY used
    Mostly used to prevent a thread from Racing itself. -> This will only happen when using Signal Handlers.

Fence:
    - it is an atomic ops that establishes a `happen-before` relationship between 2 threads WITHOUT talking about a particular memory location.
    - way to sync with all other threads that are doing a fence.
    -
*/

/*
IMP LINK: https://en.cppreference.com/w/cpp/atomic/memory_order
*/
