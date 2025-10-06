use crate::EmptyContainerError;



pub trait Last<T> {
    fn last(&self) -> Result<&T,EmptyContainerError>; 
}

impl<T> Last<T> for Vec<T> {
    fn last(&self) -> Result<&T,EmptyContainerError> {
        self.as_slice()
            .last()
            .ok_or(EmptyContainerError)
    }
}

impl<T,const N:usize> Last<T> for [T;N] {
    fn last(&self) -> Result<&T,EmptyContainerError> {
        self.as_slice()
            .last()
            .ok_or(EmptyContainerError)
    }
}

#[macro_export]
macro_rules! last_from_get {
    () => {
        fn last(&self) -> Result<& T,$crate::EmptyContainerError> {
            let sz=self.size();
            if sz.iter().any(|szi|szi == &0) {
                return Err($crate::EmptyContainerError);
            }
            let index=Index::try_from_iter(sz.into_iterator().map(|szi|szi-1)).unwrap();
            Ok(self.get(index).unwrap())
        }
    };
}