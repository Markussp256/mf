pub trait FromVec<T> {
    fn from_vec(v:Vec<T>) -> Self;
}

impl<T> FromVec<T> for Vec<T> {
    fn from_vec(v:Vec<T>) -> Self {
        v
    }
}