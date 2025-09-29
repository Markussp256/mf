use crate::IndexOutOfBoundsError;

pub trait TryRemove<T> {
    fn try_remove(&mut self,index:usize) -> Result<T,IndexOutOfBoundsError<usize>>;
}

impl<T> TryRemove<T> for Vec<T> {
    fn try_remove(&mut self, index:usize) -> Result<T,IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&self.len(),&index)?;
        Ok(self.remove(index))
    }
}