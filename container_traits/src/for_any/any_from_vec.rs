use crate::{ContainerConstructError, SizeNotEqualToRequiredSizeError};

pub trait AnyFromVec<T,Error> : Sized {
    fn any_from_vec(v:Vec<T>) -> Result<Self,Error>;
}

impl<T> AnyFromVec<T,ContainerConstructError<usize>> for Vec<T> {
    fn any_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        Ok(v)
    }
}

impl<T,const N:usize> AnyFromVec<T,ContainerConstructError<usize>> for [T;N] {
    fn any_from_vec(v:Vec<T>) -> Result<Self,ContainerConstructError<usize>> {
        v.try_into()
         .map_err(|v:Vec<T>|SizeNotEqualToRequiredSizeError::new(N,v.len()).into())
    }
}

// impl<T,S:IsLenPossible+IsLenPossible> AnyFromVec<T> for S {
//     fn any_from_vec(v:Vec<T>) -> Result<Self,Vec<T>> {
//         if Self::is_len_possible(v.len()) {
//             Ok(Self::any_from_iter(v).unwrap())
//         } else {
//             Err(v)
//         }
//     }
// }