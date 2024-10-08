use lazy_static::lazy_static;
use rand::{distributions::Alphanumeric, prelude::*};
use std::{collections::HashSet, sync::Mutex};

pub struct Robot(String);

/*
1. lazy_static!
    Rust requires static variables to be constant and fully initialized at compile time.
    A HashSet needs runtime initialization,
    So we use `lazy_static` to defer the initialization to runtime.

2. Mutex:
    Without a Mutex, multiple threads could try to modify the HashSet at the same time
    The Mutex ensures that only one thread can modify the HashSet at a time, preventing data corruption.
*/

lazy_static! {
    static ref USED_NAMES: Mutex<HashSet<String>> = Mutex::new(HashSet::new());
}

impl Robot {
    pub fn new() -> Self {
        Robot(Self::set_name())
    }

    pub fn name(&self) -> &str {
        &self.0
    }

    pub fn reset_name(&mut self) {
        self.0 = Self::set_name();
    }

    fn set_name() -> String {
        let mut hashset = USED_NAMES.lock().unwrap();
        gen_name(&mut hashset)
    }
}

fn gen_name(hashset: &mut HashSet<String>) -> String {
    loop {
        let num = rand::thread_rng().gen_range(100..=999);
        let code: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_alphabetic())
            .take(2)
            .map(|c| c.to_ascii_uppercase() as char)
            .collect();

        let name = format!("{}{}", code.to_ascii_uppercase(), num);

        if !hashset.contains(&name) {
            hashset.insert(name.clone());
            break name;
        }
    }
}
