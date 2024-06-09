use std::cell::UnsafeCell;

enum RefState {
    Unshared,
    Shared(usize),
    Exclusive,
}

pub struct RefCell<T> {
    value: UnsafeCell<T>,
    state: RefState, // How many shared references are there...
                     // But there will be only 1 Exclusive reference
}

impl<T> RefCell<T> {
    pub fn new(value: T) -> Self {
        Self {
            value: UnsafeCell::new(value),
            state: RefState::Unshared,
        }
    }

    pub fn borrow(&self) -> Option<T> {
        None
    }

    pub fn borrow_mut(&self) -> Option<&mut T> {
        None
    }
}
