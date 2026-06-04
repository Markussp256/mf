use crate::{index_iterator::ContainerIndexIterator, ContainerSize, Get};
use generic_array::{ArrayLength, GenericArray};


pub trait Iter<T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a;
}


macro_rules! iter_impl {
    () => {
        fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
            self.as_slice()
                .iter()
        }
    };
}

impl<T> Iter<T> for Vec<T> {
    iter_impl!();
}

impl<T> Iter<T> for &Vec<T> {
    iter_impl!();
}

impl<T,N:ArrayLength> Iter<T> for GenericArray<T,N> {
    iter_impl!();
}

impl<T,N:ArrayLength> Iter<T> for & mut GenericArray<T,N> {
    iter_impl!();
}

impl<T, const N:usize> Iter<T> for [T;N] {
    iter_impl!();
}

impl<T, const N:usize> Iter<T> for &[T;N] {
    iter_impl!();
}

pub fn impl_iter_from_get
    <'a,
     Index : ContainerSize,
     T : 'a,
     G : 'a+Get<Index,T>>(g:&'a G,size:Index) -> impl ExactSizeIterator<Item=&'a T> {
        ContainerIndexIterator::new_exact_size(size)
            .map(move |index:Index|g.get(index).unwrap())
}

macro_rules! impl_iter {
    ($f:ty) => {
        impl Iter<$f> for $f {
            fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a $f> where $f : 'a {
                std::iter::once(self)
            }
        }
    };
}
impl_iter!(f64);
impl_iter!(f32);
impl_iter!(i32);