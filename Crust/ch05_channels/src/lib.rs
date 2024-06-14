use std::collections::VecDeque;
use std::sync::{Arc, Condvar, Mutex};
pub struct Sender<T> {
    shared: Arc<Shared<T>>,
}
impl<T> Sender<T> {
    pub fn send(&mut self, msg: T) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.queue.push_back(msg);
        drop(inner); // Drop the lock, so the other thread can take it.
        self.shared.available.notify_one();
    }
}

impl<T> Clone for Sender<T> {
    // T -> is already clonable as we are using "Arc"
    // On every clone, need to increase no. of sender.
    fn clone(&self) -> Self {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders += 1;
        drop(inner);

        Sender {
            shared: Arc::clone(&self.shared),
        }
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let mut inner = self.shared.inner.lock().unwrap();
        inner.senders -= 1;
        let was_last = inner.senders == 0;
        drop(inner);
        // Wake up the `receiver` otherwise it will never wake up.
        if was_last {
            self.shared.available.notify_one();
        }
    }
}

// <Arc> is needed otherwise the "Sender" and "Receiver" will have 2 different instances of `Shared`... So How will they communicate...
pub struct Receiver<T> {
    shared: Arc<Shared<T>>,
    buffer: VecDeque<T>, // #optimization
}
impl<T> Receiver<T> {
    pub fn recv(&mut self) -> Option<T> {
        if let Some(t) = self.buffer.pop_front() {
            // If there is something left in `buffer`, then grab it first.
            return Some(t);
        }

        let mut inner = self.shared.inner.lock().unwrap();
        loop {
            match inner.queue.pop_front() {
                Some(msg) => {
                    if !inner.queue.is_empty() {
                        // If there is new data in `queue` then steal those data and put it in `buffer`
                        std::mem::swap(&mut self.buffer, &mut inner.queue);
                    }
                    return Some(msg);
                }
                None if inner.senders == 0 => return None,
                None => {
                    inner = self.shared.available.wait(inner).unwrap(); // Need to Drop guard in order to wait otherwise you cannot...
                }
            }
        }
    }
}

struct Inner<T> {
    queue: VecDeque<T>,
    senders: usize,
}
struct Shared<T> {
    inner: Mutex<Inner<T>>,
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
        queue: VecDeque::default(),
        senders: 1,
    };
    let shared = Shared {
        inner: Mutex::new(inner),
        available: Condvar::new(),
    };
    let shared = Arc::new(shared);

    (
        Sender {
            shared: shared.clone(),
        },
        Receiver {
            shared: shared.clone(),
            buffer: VecDeque::default(),
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn ping_pong() {
        let (mut tx, mut rx) = channel();
        tx.send(36);
        assert_eq!(rx.recv(), Some(36))
    }

    #[test]
    fn all_senders_gone() {
        let (tx, mut rx) = channel::<()>();
        drop(tx);
        assert_eq!(rx.recv(), None)
    }

    #[test]
    fn receiver_gone() {
        let (mut tx, rx) = channel::<i32>();
        drop(rx);
        tx.send(36);
    }
}
