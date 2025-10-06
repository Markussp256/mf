pub trait Empty {
    fn empty() -> Self;
}

impl<T> Empty for Vec<T> {
    fn empty() -> Self {
        Self::new()
    }
}
