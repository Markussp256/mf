use crate::IndexOutOfBoundsError;

pub trait TryInsert<T> {
    fn try_insert(&mut self,index:usize, t:T) -> Result<(),IndexOutOfBoundsError<usize>>;
}

impl<T> TryInsert<T> for Vec<T> {
    fn try_insert(&mut self, index:usize, t:T) -> Result<(),IndexOutOfBoundsError<usize>> {
        IndexOutOfBoundsError::try_new(&(self.len()+1),&index)?;
        self.insert(index,t);
        Ok(())
    }
}