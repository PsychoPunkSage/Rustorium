pub struct ThreadPool;

impl ThreadPool {
    pub fn new(threads: u32) -> ThreadPool {
        ThreadPool
    }

    // same as impl of thread::spawn
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),       // takes ownership of value and envs
        F: Send + 'static, // Send:: transfer threads from one thread to another || 'static :: receiver can hold on to the value as long as they want.
    {
        // Builder::new().spawn(f).expect("failed to spawn thread")
    }
}
