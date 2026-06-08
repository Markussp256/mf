// for static types we know the exact number of elements needed to construct an instance
// we provide the method try_take_away which removes this number of elements to construct
// the instance and returns the iterator to possible construct more stuff.

use crate::{LenNotEqualToRequiredLenError, LenTooSmallError,ContainerConstructError};
use generic_array::{ArrayLength,GenericArray};
use utils::iter::Counted;

pub trait TryFromIterator<T,E> : Sized {
    fn try_take_away<I:    Iterator<Item=T>>(iter:& mut I) -> Result<Self,E>;
    fn try_from_iter<I:IntoIterator<Item=T>>(iter:      I) -> Result<Self,E> where E : From<LenNotEqualToRequiredLenError> {
        let mut c_iter=Counted::<I::IntoIter>::new(iter.into_iter());
        let res=Self::try_take_away(& mut c_iter);
        let (iter,elems_taken)=c_iter.into_parts();
        let s=res?;
        let elems_rem=iter.count();
        if elems_rem == 0 {
            Ok(s)
        } else {
            Err(LenNotEqualToRequiredLenError::new(elems_taken,elems_taken+elems_rem).into())
        }
    }
}


impl<T, const N:usize> TryFromIterator<T, ContainerConstructError<usize>> for [T;N] {
    fn try_take_away<I:Iterator<Item=T>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        utils::iter::next_chunk(iter)
            .map_err(|e:Vec<T>|LenTooSmallError::new(N, e.len()).into())
    }
}

impl<T, N:ArrayLength> TryFromIterator<T, ContainerConstructError<usize>> for GenericArray<T,N> {
    fn try_take_away<I:Iterator<Item=T>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        utils::iter::next_chunk_gen_arr(iter)
            .map_err(|e:Vec<T>|LenTooSmallError::new(N::to_usize(), e.len()).into())
    }
}

macro_rules! impl_try_from_iter {
    ($f:ty) => {
        impl TryFromIterator<$f,ContainerConstructError<usize>> for $f {
            fn try_take_away<I:Iterator<Item=$f>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
                iter.next()
                    .ok_or(LenTooSmallError::new(1,0).into())
            }

        }
    };
}
impl_try_from_iter!(f64);
impl_try_from_iter!(f32);
impl_try_from_iter!(i32);
impl_try_from_iter!(usize);