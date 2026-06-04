
use generic_array::{ArrayLength, GenericArray};

use crate::{LenTooSmallError, LinearContainerConstructError, IntoIter};

// suggested by ChatGPT to raplace changeT and AnyFromIterator

pub trait Rebind<E> {
    type With<T> : IntoIter<T>;
    fn any_take_away<T,I:    Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<<Self as Rebind<E>>::With<T>,E>;
    fn any_from_iter<T,I:IntoIterator<Item=T>>(oref:Option<&Self>, iter:      I) -> Result<<Self as Rebind<E>>::With<T>,E>;
}

#[macro_export]
macro_rules! rebind_any_from_iter_impl {
    ($t:ty) => {
        $crate::rebind_any_from_iter_impl!($t, $crate::LinearContainerConstructError);
    };

    ($t:ty, $e:ty) => {
        fn any_from_iter<T2,I:IntoIterator<Item=T2>>(oref:Option<&Self>, iter:I) -> Result<<Self as $crate::Rebind<$e>>::With<T2>,$e> {
            let mut iter=iter.into_iter();
            let s:<Self as $crate::Rebind<$e>>::With<T2>=<Self as $crate::Rebind<$e>>::any_take_away(oref,& mut iter)?;
            $crate::for_dyn_and_stat::any_from_iterator::any_from_iter_part2_impl(s,iter)
        }
    };
}

impl<T> Rebind<LinearContainerConstructError> for Vec<T> {
    type With<T2>=Vec<T2>;
    fn any_take_away<T2,I:Iterator<Item=T2>>(oref:Option<&Self>, iter:& mut I) -> Result<Vec<T2>,LinearContainerConstructError> {
        match oref {
            None => Ok(iter.collect()),
            Some(r) => {
                let required_len=r.len();
                utils::iter::next_chunk_dyn(iter, required_len)
                .map_err(|v|LenTooSmallError::new(required_len,v.len()).into())
            }
        }
    }
    rebind_any_from_iter_impl!(T);
}

impl<T,const N:usize> Rebind<LinearContainerConstructError> for [T;N] {
    type With<T2>=[T2;N];
    fn any_take_away<T2, I: Iterator<Item=T2>>(_: Option<&Self>, iter: &mut I) -> Result<[T2;N], LinearContainerConstructError> {
        utils::iter::next_chunk(iter).map_err(|v| LenTooSmallError::new(N, v.len()).into())
    }

    rebind_any_from_iter_impl!(T);
}

impl<T,N:ArrayLength> Rebind<LinearContainerConstructError> for GenericArray<T,N> {
    type With<T2>=GenericArray<T2,N>;
    fn any_take_away<T2, I: Iterator<Item=T2>>(_: Option<&Self>, iter: &mut I) -> Result<GenericArray<T2,N>, LinearContainerConstructError> {
        utils::iter::next_chunk_gen_arr(iter).map_err(|v| LenTooSmallError::new(N::to_usize(), v.len()).into())
    }

    rebind_any_from_iter_impl!(T);
}