use crate::IndexOutOfBoundsError;

use generic_array::{ArrayLength, GenericArray};

pub trait TryMapI<Index,T> : Sized {
    fn try_map_i(self, index:Index, f:impl FnOnce(& mut T)) -> Result<Self,IndexOutOfBoundsError<Index>>;
}

macro_rules! impl_try_map_i {
    () => {
        fn try_map_i(self,index:usize, f:impl FnOnce(& mut F)) -> Result<Self,IndexOutOfBoundsError<usize>> {
            IndexOutOfBoundsError::try_new(&self.len(),&index)?;
            let mut smut=self;
            if let Some(x)=smut.get_mut(index) {
                f(x);
            }
            Ok(smut)
        }
    };
}

impl<F> TryMapI<usize, F> for Vec<F> {
    impl_try_map_i!();
}

impl<F, N:ArrayLength> TryMapI<usize, F> for GenericArray<F,N> {
    impl_try_map_i!();
}

impl<F, const N:usize> TryMapI<usize, F> for [F;N] {
    impl_try_map_i!();
}