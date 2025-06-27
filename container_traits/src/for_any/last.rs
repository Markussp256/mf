

pub trait Last<T> {
    fn last(&self) -> Option<&T>; 
}

impl<T> Last<T> for Vec<T> {
    fn last(&self) -> Option<&T> {
        self.as_slice()
            .last()
    }
}

impl<T,const N:usize> Last<T> for [T;N] {
    fn last(&self) -> Option<&T> {
        self.as_slice()
            .last()
    }
}