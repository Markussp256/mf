pub trait Concat {
    fn concat(self, rhs:Self) -> Self;
}

impl<T> Concat for Vec<T> {
    fn concat(mut self, rhs:Self) -> Self {
        self.extend(rhs);
        self
    }
}