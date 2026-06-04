
use generic_array::{ArrayLength, GenericArray};

use super::TryDiv;

macro_rules! impl_is_divable_by {
    () => {
        fn is_divable_by(&self,rhs:&T2) -> Result<(),E> {
            self.iter()
                .map(|ti|ti.is_divable_by(rhs))
                .collect()
        }
    };
}

macro_rules! impl_try_div {
    () => {
        fn try_div(self,rhs:T2) -> Result<Self::Output,E> {
            self.into_iter()
                .map(|ti|ti.try_div(rhs.clone()))
                .collect()
        }
    };
}

impl<T  : TryDiv<T2,Output = TR, Error=E>,E,
     T2 : Clone, TR> TryDiv<T2> for Vec<T> {
    type Output=Vec<TR>;
    type Error=E;
    impl_is_divable_by!();
    impl_try_div!();
}

impl<T  : TryDiv<T2,Output = TR, Error=E>,E,
     T2 : Clone, TR,
     N:ArrayLength> TryDiv<T2> for GenericArray<T,N> {
    type Output=GenericArray<TR,N>;
    type Error=E;

    impl_is_divable_by!();
    impl_try_div!();
}

impl<T  : TryDiv<T2,Output = TR, Error=E>,E,
     T2 : Clone, TR,
     const N:usize> TryDiv<T2> for [T;N] {
    type Output=[TR;N];
    type Error=E;

    impl_is_divable_by!();

    fn try_div(self,rhs:T2) -> Result<Self::Output,E> {
        self.is_divable_by(&rhs)
            .map(|_|self.map(|e|e.try_div(rhs.clone()).ok().unwrap()))
    }
}