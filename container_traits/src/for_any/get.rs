pub trait Get<Index,T> {
    fn get(&self, index:Index) -> Option<&T>;
}


impl<T> Get<usize,T> for Vec<T> {
    fn get(&self, index:usize) -> Option<&T> {
        (index < self.len()).then(||&self[index])
    }
}

impl<T, const N:usize> Get<usize,T> for [T;N] {
    fn get(&self, index:usize) -> Option<&T> {
        (index < N).then(||&self[index])
    }
}