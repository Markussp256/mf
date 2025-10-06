
pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}


impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}

impl<T,const N:usize> IsEmpty for [T;N] {
    fn is_empty(&self) -> bool {
        N == 0
    }
}