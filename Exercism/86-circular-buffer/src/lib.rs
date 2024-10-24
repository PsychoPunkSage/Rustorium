use std::fmt::Debug;

pub struct CircularBuffer<T> {
    // We fake using T here, so the compiler does not complain that
    // "parameter `T` is never used". Delete when no longer needed.
    store: Vec<T>,
    len: usize,
    cap: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T: Default + Clone + PartialEq + Debug> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        println!("Capacity: {}", capacity);
        CircularBuffer {
            store: vec![Default::default(); capacity],
            len: 0,
            cap: capacity,
        }
    }

    pub fn write(&mut self, element: T) -> Result<(), Error> {
        if self.is_full() {
            return Err(Error::FullBuffer);
        }

        if self.len < self.cap {
            self.store[self.len] = element;
            self.len += 1;
        }

        Ok(())
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.is_empty() {
            return Err(Error::EmptyBuffer);
        }
        println!("{:?}", self.store);
        let result = self.store.remove(0);
        self.store.push(T::default());
        self.len -= 1;
        println!("{:?}", result);
        println!("{:?}", self.store);
        Ok(result)
    }

    pub fn clear(&mut self) {
        self.store = vec![Default::default(); self.cap];
        self.len = 0;
    }

    pub fn overwrite(&mut self, element: T) {
        if !self.is_full() {
            let _ = self.read();
            let _ = self.write(element.clone());
        } else {
            self.store[0] = element;
        }
    }

    fn is_full(&self) -> bool {
        self.len == self.cap
    }

    fn is_empty(&self) -> bool {
        self.len == 0
    }
}
