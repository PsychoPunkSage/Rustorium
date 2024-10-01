use std::fmt::Debug;

#[derive(Clone, Debug)]
pub struct SimpleLinkedList<T> {
    data: T,
    next: Option<Box<SimpleLinkedList<T>>>,
}

impl<T: Default + PartialEq + Clone + Debug> SimpleLinkedList<T> {
    pub fn new() -> Self {
        SimpleLinkedList {
            data: Default::default(),
            next: None,
        }
    }

    // You may be wondering why it's necessary to have is_empty()
    // when it can easily be determined from len().
    // It's good custom to have both because len() can be expensive for some types,
    // whereas is_empty() is almost always cheap.
    // (Also ask yourself whether len() is expensive for SimpleLinkedList)
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut lnkd = self;
        while let Some(nxt) = &lnkd.next {
            len += 1;
            lnkd = nxt.as_ref();
        }
        len

        // let mut len = 0;
        // let mut curr = self;
        // if curr.data.is_some() {
        //     len += 1; // Count the head node
        // }
        // while let Some(ref next_node) = curr.next {
        //     len += 1;
        //     curr = next_node;
        // }
        // len
    }

    pub fn push(&mut self, element: T) {
        if self.data == Default::default() {
            self.data = element.clone();
        }
        let mut req_cell = self;
        loop {
            if req_cell.next.is_none() {
                req_cell.next = Some(Box::new(SimpleLinkedList {
                    data: element.clone(),
                    next: None,
                }));
                break;
            }
            req_cell = req_cell.next.as_mut().unwrap();
        }
    }

    pub fn pop(&mut self) -> Option<T> {
        if self.is_empty() {
            return None;
        }

        if self.next.is_none() {
            return Some(self.data.clone()); // Take the head element if it's the only node
        }

        let mut curr = self;
        while let Some(ref mut next_node) = curr.next {
            if next_node.next.is_none() {
                return curr.next.take().and_then(|n| Some(n.data)); // Take the last element
            }
            curr = next_node;
        }
        None

        // if self.len() == 1 {
        //     if self.data == Default::default() {
        //         return None;
        //     } else {
        //         let data = std::mem::take(&mut self.data);
        //         println!("Data {:?}", data);
        //         return Some(data);
        //     }
        // }

        // let mut curr = self;
        // while let Some(next) = curr.next.as_mut() {
        //     if next.next.is_none() {
        //         // Remove the last node and return its data
        //         return Some(std::mem::take(&mut next.data));
        //     }
        //     curr = next;
        // }

        // None
    }

    pub fn peek(&self) -> Option<&T> {
        todo!()
    }

    #[must_use]
    pub fn rev(self) -> SimpleLinkedList<T> {
        todo!()
    }
}

impl<T> FromIterator<T> for SimpleLinkedList<T> {
    fn from_iter<I: IntoIterator<Item = T>>(_iter: I) -> Self {
        todo!()
    }
}

// In general, it would be preferable to implement IntoIterator for SimpleLinkedList<T>
// instead of implementing an explicit conversion to a vector. This is because, together,
// FromIterator and IntoIterator enable conversion between arbitrary collections.
//
// The reason this exercise's API includes an explicit conversion to Vec<T> instead
// of IntoIterator is that implementing that interface is fairly complicated, and
// demands more of the student than we expect at this point in the track.
//
// Please note that the "front" of the linked list should correspond to the "back"
// of the vector as far as the tests are concerned.

impl<T> From<SimpleLinkedList<T>> for Vec<T> {
    fn from(mut _linked_list: SimpleLinkedList<T>) -> Vec<T> {
        todo!()
    }
}
