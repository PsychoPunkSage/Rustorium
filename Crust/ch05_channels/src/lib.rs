use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
pub struct Sender<T> {
    inner: Arc<Inner<T>>,
}
impl<T> Sender<T> {
    pub fn send(&mut self, msg: T) {
        let mut queue = self.inner.queue.lock().unwrap();
        queue.push_back(msg);
        drop(queue); // Drop the lock, so the other thread can take it.
        self.inner.available.notify_one();
    }
}
// <Arc> is needed otherwise the "Sender" and "Receiver" will have 2 different instances of `Inner`... So How will they communicate...
pub struct Receiver<T> {
    inner: Arc<Inner<T>>,
}
impl<T> Receiver<T> {
    pub fn recv(&mut self) -> T {
        let mut queue = self.inner.queue.lock().unwrap();
        loop {
            match queue.pop_front() {
                Some(msg) => return msg,
                None => {
                    queue = self.inner.available.wait(queue).unwrap(); // Need to Drop guard in order to wait otherwise you cannot...
                }
            }
        }
    }
}
struct Inner<T> {
    queue: Mutex<VecDeque<T>>,
    available: Condvar,
}
/*
Condvar::>
    Let go of Mutex at the same time you wake up the thread.
        - Otherwise, if you wake up (before releasing the lock)
        - the threads will wake up but they can't access the lock and will go to sleep.
        - An then we will release the lock BUT no one is there to take the lock, hence, results in `Deadlock`.
*/

/*
Mutex:
    - Here OS put the thread back to sleep and wake it up when the Mutex is available.
    - Efficient but adds latency.

Boolean Semaphore:
    - Suppose someone has a lock... so we need to repeatedly check when the lock is lifted.
*/

pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Inner {
        queue: Mutex::default(),
    };
    let inner = Arc::new(inner);
    (
        Sender {
            inner: inner.clone(),
        },
        Receiver {
            inner: inner.clone(),
        },
    )
}
