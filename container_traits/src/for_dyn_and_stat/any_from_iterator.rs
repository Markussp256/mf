use crate::{LenNotEqualToRequiredLenError, LenTooSmallError, LinearContainerConstructError};

use generic_array::{GenericArray, ArrayLength};

use utils::iter::Counted;

// due to Concatenated we pass a reference to an instance of type Self
// so that size information can be extracted

// any_take_away takes elements form the iterator possible leaving some element in the iterator left
// any_from_iter is supposed to take all elements from the iterator

pub trait AnyFromIterator<T,E> : Sized {
    
    fn any_take_away<I:    Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,E>;

    fn any_from_iter<I:IntoIterator<Item=T>>(oref:Option<&Self>, iter:      I) -> Result<Self,E> where LenNotEqualToRequiredLenError : Into<E> {
        let mut c_iter=Counted::<I::IntoIter>::new(iter.into_iter());
        let res=Self::any_take_away(oref,& mut c_iter);
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



impl<T> AnyFromIterator<T,LinearContainerConstructError> for Vec<T> {
    fn any_take_away<I:Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,LinearContainerConstructError> {
        match oref {
            None => Ok(iter.collect()),
            Some(r) => {
                let required_len = r.len();
                utils::iter::next_chunk_dyn(iter, required_len)
                    .map_err(|v|LenTooSmallError::new(required_len, v.len()).into())
            }
        }
    }
}

// Array
impl<T, const N:usize> AnyFromIterator<T, LinearContainerConstructError> for [T; N] {
    fn any_take_away<I: Iterator<Item=T>>(_: Option<&Self>, iter: &mut I) -> Result<Self, LinearContainerConstructError> {
        utils::iter::next_chunk(iter).map_err(|v| LenTooSmallError::new(N, v.len()).into())
    }
}

// GenericArray
impl<T, N: ArrayLength> AnyFromIterator<T, LinearContainerConstructError> for GenericArray<T, N> {
    fn any_take_away<I: Iterator<Item=T>>(_: Option<&Self>, iter: &mut I) -> Result<Self, LinearContainerConstructError> {
        utils::iter::next_chunk_gen_arr(iter).map_err(|v| LenTooSmallError::new(N::to_usize(), v.len()).into())
    }
}


macro_rules! impl_any_from_iter {
    ($f:ty) => {
        impl AnyFromIterator<$f,LinearContainerConstructError> for $f {
            fn any_take_away<I:Iterator<Item=$f>>(_:Option<&Self>, iter:& mut I) -> Result<Self,LinearContainerConstructError> {
                iter.next()
                    .ok_or(LenTooSmallError::new(1,0).into())
            }
        }
    };
}
impl_any_from_iter!(f64);
impl_any_from_iter!(f32);
impl_any_from_iter!(i32);
impl_any_from_iter!(usize);