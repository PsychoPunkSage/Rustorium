// use rayon::prelude::*;
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    thread,
};

// pub fn frequency(input: &[&str], _worker_count: usize) -> HashMap<char, usize> {
//     input
//         .par_iter()
//         .map(|&lines| {
//             lines
//                 .to_lowercase()
//                 .chars()
//                 .into_iter()
//                 .fold(HashMap::new(), |mut acc, c| {
//                     if c.is_alphabetic() {
//                         *acc.entry(c).or_insert(0) += 1;
//                     }
//                     acc
//                 })
//         })
//         .reduce(HashMap::new, |mut acc, map| {
//             map.into_iter()
//                 .for_each(|(key, val)| *acc.entry(key).or_insert(0) += val);
//             acc
//         })
// }
pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let chunk_size = (input.len() + worker_count - 1) / worker_count;
    let op_vec = input
        .chunks(chunk_size)
        .map(|chunk| chunk.iter().map(|&s| s.to_string()).collect())
        .collect::<Vec<Vec<String>>>();

    let res = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    op_vec.into_iter().for_each(|chunk| {
        let res = Arc::clone(&res);

        let handle = thread::spawn(move || {
            let mut local_map = HashMap::new();

            chunk.iter().for_each(|line| {
                line.to_lowercase().chars().for_each(|c| {
                    if c.is_alphabetic() {
                        *local_map.entry(c).or_insert(0) += 1;
                    }
                });
            });

            let mut global_map = res.lock().unwrap();
            *global_map =
                local_map
                    .into_iter()
                    .fold(global_map.clone(), |mut acc, (key, value)| {
                        *acc.entry(key).or_insert(0) += value;
                        acc
                    });
        });

        handles.push(handle);
    });

    handles.into_iter().for_each(|h| h.join().unwrap());

    Arc::try_unwrap(res).unwrap().into_inner().unwrap()
}
