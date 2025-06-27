use std::ops::Mul;
use num_traits::One;

pub trait IntoProduct : Sized+IntoIterator where Self::Item : One {
    fn into_product(self) -> Self::Item {
        self.into_iter()
            .reduce(Mul::mul)
            .unwrap_or(Self::Item::one())
    }
}

impl<T:One, I:Sized+IntoIterator<Item=T>> IntoProduct for I {}