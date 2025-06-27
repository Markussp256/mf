use crate::{LenTooSmallError, LinearContainerConstructError};

// equivalent as try_from_iterator but for optimization

// take away takes elements form the iterator possible leaving some element in the iterator left
// from_iter is supposed to take all elements from the iterator

pub trait TryFromLocalParameters<T,E> : Sized {
    fn try_take_away<I:    Iterator<Item=T>>(self, iter:& mut I) -> Result<Self,E>;
    fn try_from_iter<I:IntoIterator<Item=T>>(self, iter:      I) -> Result<Self,E>;
}


#[macro_export]
macro_rules! try_from_local_parameters_impl {
    ($t:ty) => {
        $crate::try_from_local_parameters_impl!($t, $crate::LinearContainerConstructError);
    };

    ($t:ty, $e:ty) => {
        fn try_from_iter<I:IntoIterator<Item=$t>>(self, iter:I) -> Result<Self,$e> {
            let vs:Vec<$t>=iter.into_iter().collect();
            let provided_len=vs.len();
            let mut iter=vs.into_iter();
            let s=<Self as $crate::TryFromLocalParameters<$t,$e>>::try_take_away(self,& mut iter)?;
            let left=iter.count();
            if left == 0 {
                Ok(s)
            } else {
                Err($crate::LenNotEqualToRequiredLenError::new(provided_len-left,provided_len).into())
            }
        }
    };
}

#[macro_export]
macro_rules! impl_try_from_local_parameters_for_multiplicative_group {
    () => {
        fn try_take_away<I:    Iterator<Item=F>>(self, iter:& mut I) -> Result<Self,E> {
            <Self as container_traits::AnyFromParameters<F>>::any_take_away(Some(&self),iter)
                .map(|rhs|self*rhs)
        }

        $crate::try_from_local_parameters_impl!(F);
    };
}


macro_rules! impl_try_from_iter {
    ($f:ty) => {
        impl TryFromLocalParameters<$f,LinearContainerConstructError> for $f {
            fn try_take_away<I:Iterator<Item=$f>>(self, iter:& mut I) -> Result<Self,LinearContainerConstructError> {
                iter.next()
                    .ok_or(LenTooSmallError::new(1,0).into())
                    .map(|d|self+d)
            }

            try_from_local_parameters_impl!($f);
        }
    };
}
impl_try_from_iter!(f64);
impl_try_from_iter!(f32);
impl_try_from_iter!(i32);
impl_try_from_iter!(usize);