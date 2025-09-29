
// returns corresponding type that holds container of provided length
// same if dynamic
pub trait ChangeLen {
    type Output<const L:usize>;
}

impl<T> ChangeLen for Vec<T> {
    type Output<const L:usize> = [T;L];
}

impl<T, const N:usize> ChangeLen for [T;N] {
    type Output<const L:usize> = [T;L];
}