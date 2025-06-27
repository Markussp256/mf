pub trait TryInsert<T> {
    fn try_insert(&mut self,index:usize, t:T) -> Option<()>;
}

impl<T> TryInsert<T> for Vec<T> {
    fn try_insert(&mut self, index:usize, t:T) -> Option<()> {
        (index <= self.len()).then(||self.insert(index,t))
    }
}