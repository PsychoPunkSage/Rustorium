pub fn find(array: &[i32], key: i32) -> Option<usize> {
    if array.is_empty() {
        return None;
    }

    let mid = array.len() / 2;
    let init = array[mid];

    if init == key {
        Some(mid)
    } else if init < key {
        // Search in the right half, adjust index by adding `mid + 1`
        find(&array[mid + 1..], key).map(|index| mid + 1 + index)
        /*
        return Some(mid + find(&array[mid..], key).unwrap());
            - If the recursive call returns None, unwrapping it (.unwrap()) will cause the program to panic

        find(&array[mid + 1..], key).map(|index| mid + 1 + index)
            - If the recursive call returns None, the map operation will also return None, thus avoiding a panic.
        */
    } else {
        // Search in the left half
        find(&array[..mid], key)
    }
}
