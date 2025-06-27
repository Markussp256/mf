pub trait Empty {
    fn empty() -> Self;

    fn is_empty(&self) -> bool;
}

impl<T> Empty for Vec<T> {
    fn empty() -> Self {
        Self::new()
    }

    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}