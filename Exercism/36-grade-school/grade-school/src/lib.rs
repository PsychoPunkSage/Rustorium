use std::collections::BTreeMap;

pub struct School {
    data: BTreeMap<i32, Vec<String>>,
}

impl School {
    pub fn new() -> School {
        School {
            data: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        let students = self.data.entry(grade as i32).or_insert(Vec::new());

        if !students.contains(&(student.to_string())) {
            students.push(student.to_string());
            self.data.entry(grade as i32).or_insert(Vec::new()).sort();
        }
    }

    pub fn grades(&self) -> Vec<u32> {
        self.data.keys().map(|&k| k as u32).collect::<Vec<u32>>()
    }

    // If `grade` returned a reference, `School` would be forced to keep a `Vec<String>`
    // internally to lend out. By returning an owned vector of owned `String`s instead,
    // the internal structure can be completely arbitrary. The tradeoff is that some data
    // must be copied each time `grade` is called.
    pub fn grade(&self, grade: u32) -> Vec<String> {
        self.data.get(&(grade as i32)).cloned().unwrap_or_default()
    }
}
