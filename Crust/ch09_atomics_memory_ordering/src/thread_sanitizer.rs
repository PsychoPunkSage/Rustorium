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
