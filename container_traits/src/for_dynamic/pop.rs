pub trait Pop<T> {
    fn pop(& mut self) -> Option<T>; 
}

impl<T> Pop<T> for Vec<T> {
    fn pop(& mut self) -> Option<T> {
        self.pop()
    }
}