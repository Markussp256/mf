pub trait NumberOfDegreesOfFreedom<T> {
    fn ndofs(&self) -> usize;
}

impl<T> NumberOfDegreesOfFreedom<T> for Vec<T> {
    fn ndofs(&self) -> usize {
        self.len()
    }
}

impl<T,const N:usize> NumberOfDegreesOfFreedom<T> for [T;N] {
    fn ndofs(&self) -> usize {
        N
    }
}