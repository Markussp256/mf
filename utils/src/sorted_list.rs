
#[derive(Clone,
         Debug,
         derive_more::Index,
         derive_more::IntoIterator)]
pub struct SortedList<T> (Vec<T>);

impl<T> SortedList<T> {
    pub fn new_unchecked(v: Vec<T>) -> Self {
        Self(v)
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }
}

impl<T:Ord> SortedList<T> {
    pub fn new(v : Vec<T>) -> Self {
        let mut vmut=v;
        vmut.sort();
        Self(vmut)
    }

    pub fn try_push(& mut self, t:T) -> Result<(),()> {
        if self.0.is_empty() || &t > self.0.last().unwrap() {
            self.0.push(t);
            Ok(())
        } else {
            Err(())
        }
    }
}