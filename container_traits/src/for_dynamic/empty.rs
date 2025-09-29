pub trait Empty {
    fn empty() -> Self;
}

pub trait IsEmpty {
    fn is_empty(&self) -> bool;
}

impl<T> Empty for Vec<T> {
    fn empty() -> Self {
        Self::new()
    }
}

impl<T> IsEmpty for Vec<T> {
    fn is_empty(&self) -> bool {
        self.is_empty()
    }
}