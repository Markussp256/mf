use std::ops::Add;
use num_traits::Zero;

pub trait IntoSum : Sized+IntoIterator where Self::Item : Zero {
    fn into_sum(self) -> Self::Item {
        self.into_iter()
            .reduce(Add::add)
            .unwrap_or(Self::Item::zero())
    }
}

impl<T:Zero,I:Sized+IntoIterator<Item=T>> IntoSum for I {}