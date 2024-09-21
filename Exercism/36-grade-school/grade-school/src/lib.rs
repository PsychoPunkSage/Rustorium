use std::collections::BTreeMap;

pub struct School {
    data: BTreeMap<i32, Vec<String>>,
}

/*
BETTER Approach

pub struct School(BTreeMap<i32, BTreeSet(String));
*/

impl School {
    pub fn new() -> School {
        School {
            data: BTreeMap::new(),
        }
    }

    pub fn add(&mut self, grade: u32, student: &str) {
        if self
            .data
            .values()
            .any(|students| students.contains(&student.to_string()))
        {
            return;
        }

        let students = self.data.entry(grade as i32).or_insert(Vec::new());
        students.push(student.to_string());
        students.sort()
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
