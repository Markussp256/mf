pub trait First<T> {
    fn first(&self) -> Option<&T>;
}

impl<T> First<T> for Vec<T> {
    fn first(&self) -> Option<&T> {
        self.as_slice()
            .first()
    }
}

impl<T,const N:usize> First<T> for [T;N] {
    fn first(&self) -> Option<&T> {
        self.as_slice()
            .first()
    }
}