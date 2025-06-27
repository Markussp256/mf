
// equivalent to IntoIterator
// but necessary because nalgbra types do not implement IntoIterator
// we call the function into_iterator to distinguish from the fn into_iter
// which is typically implemented

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



// pub trait Iter : AsSlice {
//     fn iter<'a>(&'a self) -> impl Iterator<Item=&'a T> {
//         self.as_slice()
//             .iter()
//     }
// }