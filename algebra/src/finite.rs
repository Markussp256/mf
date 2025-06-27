use std::ops::{Add,Sub, Neg, Mul, Div};
use num_traits::Inv;

macro_rules! impl_un_op {
    ($fn:ident, $tr:ident) => {
        paste::paste!(
        pub fn [<try_ $fn>](self) -> Option<Self> where T:$tr<Output=T> {
            Self::try_new(self.0.$fn())
        });
    };
}

macro_rules! impl_bin_op {
    ($fn:ident, $tr:ident) => {
        paste::paste!(
        pub fn [<try_ $fn>](self, rhs:Self) -> Option<Self> where T:$tr<Output=T> {
            Self::try_new(self.0.$fn(rhs.0))
        });
    };
}

pub struct Finite<T>(T);

impl<T> Finite<T> {
    pub fn try_new(t:T) -> Option<Self> {
        t.is_finite().then(||Self(t))
    }

    impl_bin_op!(add, Add);
    impl_bin_op!(sub, Sub);
    impl_bin_op!(mul, Mul);
    impl_bin_op!(div, Div);
    impl_un_op!(neg, Neg);
    impl_un_op!(inv, Inv);
}