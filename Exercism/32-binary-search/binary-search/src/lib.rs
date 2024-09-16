pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mid = array.len() / 2;
    let init = array[mid]; // middle value

    if mid == 0 && init != key {
        return None;
    }

    if init == key {
        return Some(mid);
    } else if init < key {
        return Some(mid + find(&array[mid..], key).unwrap());
    } else {
        return find(&array[..mid], key);
    }
}

// pub fn find(array: &[i32], key: i32) -> Option<usize> {
//     if array.is_empty() {
//         return None;
//     }

//     let mid = array.len() / 2;
//     let init = array[mid];

//     if init == key {
//         Some(mid)
//     } else if init < key {
//         // Search in the right half, adjust index by adding `mid + 1`
//         find(&array[mid + 1..], key).map(|index| mid + 1 + index)
//     } else {
//         // Search in the left half
//         find(&array[..mid], key)
//     }
// }
