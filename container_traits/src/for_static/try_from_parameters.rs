// for static types we know the exact number of elements needed to construct an instance
// we provide the method try_take_away which removes this number of elements to construct
// the instance and returns the iterator to possible construct more stuff.

use crate::{LenTooSmallError, ContainerConstructError};

pub trait TryFromParameters<T,E> : Sized {
    fn try_take_away<I:    Iterator<Item=T>>(iter:& mut I) -> Result<Self,E>;
    fn try_from_iter<I:IntoIterator<Item=T>>(iter:      I) -> Result<Self,E>;
}

#[macro_export]
macro_rules! try_from_parameters_impl {
    ($t:ty) => {
        $crate::try_from_parameters_impl!($t, $crate::ContainerConstructError<usize>);
    };

    ($t:ty, $e:ty) => {
        fn try_from_iter<I:IntoIterator<Item=$t>>(iter:I) -> Result<Self,$e> {
            let vs:Vec<$t>=iter.into_iter().collect();
            let provided_len=vs.len();
            let mut iter=vs.into_iter();
            let s=<Self as $crate::for_static::TryFromParameters<$t,$e>>::try_take_away(& mut iter)?;
            let left=iter.count();
            if left == 0 {
                Ok(s)
            } else {
                Err($crate::LenNotEqualToRequiredLenError::new(provided_len-left,provided_len).into())
            }
        }
    };
}

impl<T, const N:usize> TryFromParameters<T, ContainerConstructError<usize>> for [T;N] {
    fn try_take_away<I:Iterator<Item=T>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
        utils::iter::next_chunk(iter)
            .map_err(|e:Vec<T>|LenTooSmallError::new(N, e.len()).into())
    }
    try_from_parameters_impl!(T);
}

macro_rules! impl_try_from_iter {
    ($f:ty) => {
        impl TryFromParameters<$f,ContainerConstructError<usize>> for $f {
            fn try_take_away<I:Iterator<Item=$f>>(iter:& mut I) -> Result<Self,ContainerConstructError<usize>> {
                iter.next()
                    .ok_or(LenTooSmallError::new(1,0).into())
            }

            try_from_parameters_impl!($f);
        }
    };
}
impl_try_from_iter!(f64);
impl_try_from_iter!(f32);
impl_try_from_iter!(i32);
impl_try_from_iter!(usize);