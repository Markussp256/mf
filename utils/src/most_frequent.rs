
// returns element that occurs most frequent

// Hashmap would maybe be better
// but would require T to implement Eq+Hash

// returns index of first occurence
// can sometimes avoid borrowship problems
pub fn most_frequent_index<T: PartialEq>(v: &[T]) -> Option<usize> {
    let mut counts: Vec<(usize, usize)> = Vec::new();
    for i in 0..v.len() {
        match counts.iter_mut().find(|(j, _)| v[i] == v[*j]) {
            Some((_, rc)) => *rc += 1,
            None => counts.push((i, 1)),
        }
    }
    counts
        .into_iter()
        .max_by_key(|(_, count)| *count)
        .map(|(value, _)| value)
}


pub fn most_frequent<T: PartialEq>(v: &[T]) -> Option<&T> {
    most_frequent_index(v)
        .map(|i|&v[i])
}


