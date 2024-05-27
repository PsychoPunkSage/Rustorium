use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        Worker {
            id,
            thread: thread::spawn(move || loop {
                // loop:: cause we want our `thread` to constantly look for JOBS
                let job = receiver.lock().unwrap().recv().unwrap();
                println!("WORKER:{} assigned a JOB; executing....", id);
                job();
            }),
        }
    }
}

// Holds on the `closures` that we want to send down the `channel`.
// Wee need ALL TYPES OF JOBS to get executed
type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Create a new thread pool
    ///
    /// the `num_threads` is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the `num_threads` is 0 or less.
    pub fn new(num_threads: usize) -> ThreadPool {
        assert!(num_threads > 0);

        let (sx, rx) = mpsc::channel();
        let rx = Arc::new(Mutex::new(rx));

        let mut workers = Vec::with_capacity(num_threads);
        for id in 0..num_threads {
            // create threads
            /*
            We will create another structs called `WORKER`

            => Thread spawning will implement a `closure` immediately after thread creation...  we don't want that...
            => We want the thread to only execute when it is assigned some work

            ***
            => We want workers to have shared ownership of `rx`
                * Arc   :: Thread-safe multiple ownership
                * Mutex :: Thread-safe mutability
             */
            workers.push(Worker::new(id, Arc::clone(&rx)));
        }

        ThreadPool {
            workers,
            sender: sx,
        }
    }

    // same as impl of thread::spawn
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce(),       // takes ownership of value and envs
        F: Send + 'static, // Send:: transfer threads from one thread to another || 'static :: receiver can hold on to the value as long as they want.
    {
        // We want to sen closures to different threads
        // Best way:: Channels

        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}
