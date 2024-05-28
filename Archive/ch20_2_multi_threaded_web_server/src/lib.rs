use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

enum Message {
    NewJob(Job),
    Terminate,
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Message>,
}
impl Drop for ThreadPool {
    fn drop(&mut self) {
        println!("Sending Terminate message to Workers");

        for _ in &mut self.workers {
            self.sender.send(Message::Terminate).unwrap();
        }

        println!("Shutting down workers");

        for worker in &mut self.workers {
            println!("Shutting down Worker:{}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
            // worker.thread.take().unwrap().join().unwrap();
        }
    }
}

struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}
impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Message>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            // loop:: cause we want our `thread` to constantly look for JOBS
            let message = receiver.lock().unwrap().recv().unwrap();
            match message {
                Message::NewJob(job) => {
                    println!("WORKER:{} assigned a JOB; executing....", id);
                    job();
                }
                Message::Terminate => {
                    println!("WORKER:{} Shutting Down....", id);
                    break;
                }
            };
        });

        Worker {
            id,
            thread: Some(thread),
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
        self.sender.send(Message::NewJob(job)).unwrap();
    }
}
