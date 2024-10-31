// Struct to describe Binary tree... adds integer element in binary tree + search element in binary tree

fn main() {
    let mut tree = Node::new(123);
    tree.insert(1231);
    tree.insert(12);
    println!("{:?}", tree);
    println!("{}", tree.search(1231));
    println!("{}", tree.search(1232));
}

#[derive(Debug)]
struct Node {
    // node left right
    value: i32,
    left: Option<Box<Node>>,  // Try to use `Box` somehow
    right: Option<Box<Node>>, // Try to use `Box` somehow
}

impl Node {
    fn new(value: i32) -> Self {
        Node {
            value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        /*
        1. cmp val... value > self.value : right || else left,
        2. if any val == None place it .... coninut cmp
         */

        if self.value >= value {
            match self.left {
                None => self.left = Some(Box::new(Node::new(value))),
                Some(ref mut left_child) => left_child.insert(value),
            }
        } else {
            match self.right {
                None => self.right = Some(Box::new(Node::new(value))),
                Some(ref mut right_child) => right_child.insert(value),
            }
        }
    }

    fn search(&self, value: i32) -> bool {
        /*
        1. self.val == value GOOD
        2. recursive call search...
        3. if DNE ??
         */

        if self.value == value {
            return true;
        } else if value < self.value {
            match self.left {
                None => false,
                Some(ref left_child) => left_child.search(value),
            }
        } else {
            match self.right {
                None => false,
                Some(ref right_child) => right_child.search(value),
            }
        }
    }
}
