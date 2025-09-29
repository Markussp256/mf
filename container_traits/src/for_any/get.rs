use crate::IndexOutOfBoundsError;

pub trait Get<Index,T> {
    fn get(&self, index:Index) -> Result<&T,IndexOutOfBoundsError<Index>>;
}


impl<T> Get<usize,T> for Vec<T> {
    fn get(&self, index:usize) -> Result<&T,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.len(), &index)?;
        Ok(&self[index])
    }
}

impl<T, const N:usize> Get<usize,T> for [T;N] {
    fn get(&self, index:usize) -> Result<&T,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.len(), &index)?;
        Ok(&self[index])
    }
}