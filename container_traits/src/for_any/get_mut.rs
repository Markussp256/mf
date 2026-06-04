use generic_array::{ArrayLength, GenericArray};

use crate::{ContainerIndex, IndexOutOfBoundsError};

pub trait GetMut<Index,T> {
    fn get_mut(&mut self, index:Index) -> Result<&mut T,IndexOutOfBoundsError<Index>>;
}

macro_rules! impl_get_mut {
    () => {
        fn get_mut(&mut self, index:usize) -> Result<&mut T,IndexOutOfBoundsError<usize>> {
            IndexOutOfBoundsError::try_new(&self.len(),&index)?;
            Ok(& mut self[index])
        }
    };
}

impl<T> GetMut<usize,T> for Vec<T> {
    impl_get_mut!();
}

impl<T,N : ArrayLength> GetMut<usize,T> for GenericArray<T,N> {
    impl_get_mut!();
}

impl<T, const N:usize> GetMut<usize,T> for [T;N] {
    impl_get_mut!();
}

macro_rules! impl_get_mut {
    () => {
        fn get_mut(&mut self, index:(usize,Index)) -> Result<&mut T,IndexOutOfBoundsError<(usize,Index)>> {
            let len=self.len();
            if len == 0 {
                return Err(IndexOutOfBoundsError::new(&(0,Index::default()),&index))
            }
            let index0=index.0.clone();
            self.get_mut(index.0).ok().unwrap()
                   .get_mut(index.1)
                   .map_err(|ioob|
                    {
                        let (bounds1,index1)=ioob.into_parts();
                        IndexOutOfBoundsError::new(&(len,bounds1),&(index0,index1))
                    })
                    
            //if index.0 >= self.len() { return None; }
            //self[index.0].get_mut(index.1)
        } 
    };
}

impl<Index : Default, T,S:GetMut<Index,T>> GetMut<(usize,Index),T> for Vec<S> where (usize,Index) : ContainerIndex {
    impl_get_mut!();
}

impl<Index : Default, T, S:GetMut<Index,T>,const N:usize> GetMut<(usize,Index),T> for [S;N] where (usize,Index) : ContainerIndex {
    impl_get_mut!();
}