pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T>
where
    T: Messenger,
{
    messenger: &'a T,
    value: usize,
    max: usize,
}

impl<'a, T> LimitTracker<'a, T>
where
    T: Messenger,
{
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max,
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;

        let percentage_usage = self.value as f64 / self.max as f64;

        if percentage_usage >= 1.0 {
            self.messenger.send("Error: You are over your quota!");
        } else if percentage_usage >= 0.9 {
            self.messenger.send("Warning! You used 90% of your quota");
        } else if percentage_usage >= 0.75 {
            self.messenger.send("You used 75% of your quota");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>,
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![]),
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_an_over_75_percent_warning_message() {
        let mock = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock, 10);

        limit_tracker.set_value(80);
        assert_eq!(mock.sent_messages.borrow().len(), 1); // need immutable reference to calculate `len`
    }
}
