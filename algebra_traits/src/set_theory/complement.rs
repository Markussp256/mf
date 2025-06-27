pub trait Complement {
    fn complement(self) -> Self;
}

impl<T:Clone+PartialEq+strum::IntoEnumIterator> Complement for Vec<T> {
    fn complement(self) -> Self {
        T::iter()
        .filter(|t|!self.contains(&t))
        .collect()
    }
}