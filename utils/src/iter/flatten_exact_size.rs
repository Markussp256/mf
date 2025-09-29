use std::iter::{Flatten, Take, Iterator};

use super::{WithExactSize,IntoExactSizeIterator};

pub trait FlattenExactSize : Sized+ExactSizeIterator
    where Self::Item : ExactSizeIterator,
    for<'a> &'a Self : IntoIterator<Item=&'a Self::Item> {
    fn flatten_exact_size(self) -> WithExactSize<Take<Flatten<Self>>> {
        let self_ref=&self;
        let sz=
            self_ref
                .into_iter()
                .map(|iter|iter.len())
                .sum();
        self.flatten()
            .into_exact_size_iter(sz)
    }
}
impl<A:Sized+ExactSizeIterator> FlattenExactSize for A
    where A::Item : ExactSizeIterator,
    for<'a> &'a Self : IntoIterator<Item=&'a Self::Item> {}
