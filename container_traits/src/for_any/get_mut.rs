pub trait GetMut<Index,T> {
    fn get_mut(&mut self, index:Index) -> Option<&mut T>;
}


impl<T> GetMut<usize,T> for Vec<T> {
    fn get_mut(&mut self, index:usize) -> Option<&mut T> {
        (index < self.len()).then(||& mut self[index])
    }
}

impl<T, const N:usize> GetMut<usize,T> for [T;N] {
    fn get_mut(&mut self, index:usize) -> Option<&mut T> {
        (index < N).then(||& mut self[index])
    }
}

macro_rules! impl_get_mut {
    () => {
        fn get_mut(&mut self, index:(usize,Index)) -> Option<&mut T> {
            self.get_mut(index.0)
                .and_then(|si|si.get_mut(index.1))
            //if index.0 >= self.len() { return None; }
            //self[index.0].get_mut(index.1)
        } 
    };
}

impl<Index,T,S:GetMut<Index,T>> GetMut<(usize,Index),T> for Vec<S> {
    impl_get_mut!();
}

impl<Index,T, S:GetMut<Index,T>,const N:usize> GetMut<(usize,Index),T> for [S;N] {
    impl_get_mut!();
}