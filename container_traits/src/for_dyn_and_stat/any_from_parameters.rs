use crate::{LenTooSmallError, LinearContainerConstructError};

// equivalent as any_from_iterator but for optimization

// take away takes elements form the iterator possible leaving some element in the iterator left
// from_iter is supposed to take all elements from the iterator

pub trait AnyFromParameters<T,E> : Sized {
    fn any_take_away<I:Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,E>;
    fn any_from_iter<I:IntoIterator<Item=T>>(oref:Option<&Self>, iter:I) -> Result<Self,E>;
}

// pub fn any_from_parameters_part2_impl<S:IntoParameters<T>,T,E>(s:S,mut iter:impl Iterator<Item=T>) -> Result<S,E> where LenNotEqualToRequiredLenError : Into<E> {
//     if iter.next().is_none() {
//         Ok(s)
//     } else {
//         let required_len=s.into_parameters().count();
//         let provided_len=required_len+1+iter.count();
//         Err(LenNotEqualToRequiredLenError::new(required_len,provided_len).into())
//     }
// }

#[macro_export]
macro_rules! any_from_parameters_impl {
    ($t:ty) => {
        $crate::any_from_parameters_impl!($t, $crate::LinearContainerConstructError);
    };

    ($t:ty, $e:ty) => {
        fn any_from_iter<I:IntoIterator<Item=$t>>(oref:Option<&Self>, iter:I) -> Result<Self,$e> {
            let vs:Vec<$t>=iter.into_iter().collect();
            let provided_len=vs.len();
            let mut iter=vs.into_iter();
            let s=<Self as $crate::AnyFromParameters<$t,$e>>::any_take_away(oref,& mut iter)?;
            let left=iter.count();
            if left == 0 {
                Ok(s)
            } else {
                Err($crate::LenNotEqualToRequiredLenError::new(provided_len-left,provided_len).into())
            }
        }
    };
}

impl<T> AnyFromParameters<T,LinearContainerConstructError> for Vec<T> {
    fn any_take_away<I:Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,LinearContainerConstructError> {
        match oref {
            None => Ok(iter.collect()),
            Some(r) => {
                let required_len=r.len();
                utils::iter::next_chunk_dyn(iter, required_len)
                .map_err(|v|LenTooSmallError::new(required_len,v.len()).into())
            }
        }
    }
    any_from_parameters_impl!(T);
}

impl<T, const N:usize> AnyFromParameters<T,LinearContainerConstructError> for [T;N] {
    fn any_take_away<I:Iterator<Item=T>>(_:Option<&Self>, iter:& mut I) -> Result<Self,LinearContainerConstructError> {
        utils::iter::next_chunk(iter)
            .map_err(|v|LenTooSmallError::new(N,v.len()).into())
    }
    
    any_from_parameters_impl!(T);
}

macro_rules! impl_any_from_iter {
    ($f:ty) => {
        impl AnyFromParameters<$f,LinearContainerConstructError> for $f {
            fn any_take_away<I:Iterator<Item=$f>>(_:Option<&Self>, iter:& mut I) -> Result<Self,LinearContainerConstructError> {
                iter.next()
                    .ok_or(LenTooSmallError::new(1,0).into())
            }

            any_from_parameters_impl!($f);
        }
    };
}
impl_any_from_iter!(f64);
impl_any_from_iter!(f32);
impl_any_from_iter!(i32);
impl_any_from_iter!(usize);