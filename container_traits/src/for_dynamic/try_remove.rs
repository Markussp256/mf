pub trait TryRemove<T> {
    fn try_remove(&mut self,index:usize) -> Option<T>;
}

impl<T> TryRemove<T> for Vec<T> {
    fn try_remove(&mut self, index:usize) -> Option<T> {
        (index < self.len()).then(||self.remove(index))
    }
}