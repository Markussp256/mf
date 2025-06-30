
// equivalent to IntoIterator
// but necessary because nalgbra types do not implement IntoIterator
// we call the function into_iterator to distinguish from the fn into_iter
// which is typically implemented

use utils::iter::IntoExactSizeIterator;

pub trait IntoIter<T> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T>;
}

macro_rules! iter_impl {
    () => {
        fn into_iterator(self) -> impl ExactSizeIterator<Item=T> {
            self.into_iter()
        }
    };
}

impl<T> IntoIter<T> for Vec<T> {
    iter_impl!();
}

impl<T, const N:usize> IntoIter<T> for [T;N] {
    iter_impl!();
}

macro_rules! impl_into_iter {
    ($f:ty) => {
        impl IntoIter<$f> for $f {
            fn into_iterator(self) -> impl ExactSizeIterator<Item=$f> {
                std::iter::once(self)
            }
        }
    };
}
impl_into_iter!(f64);
impl_into_iter!(f32);
impl_into_iter!(i32);

pub fn zip<TLhs,TRhs>(
    lhs:impl IntoIter<TLhs>,
    rhs:impl IntoIter<TRhs>) -> impl ExactSizeIterator<Item=(TLhs,TRhs)>
{
    let lhs_iter=lhs.into_iterator();
    let rhs_iter=rhs.into_iterator();
    let len=usize::min(lhs_iter.len(),rhs_iter.len());
    lhs_iter.zip(rhs_iter)
            .into_exact_size_iter(len)
}

// pub trait Iter : AsSlice {
//     fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> {
//         self.as_slice()
//             .iter()
//     }
// }