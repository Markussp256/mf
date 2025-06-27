use crate::{EnhancedArray,EnhancedContainer};

pub type EnhancedVec<T>=EnhancedContainer<Vec<T>>;
use container_traits::IntoInner;

impl<T> Into<Vec<T>> for EnhancedVec<T> {
    fn into(self) -> Vec<T> {
        self.into_inner()
    }
}

utils::from_via!(    impl<T, const N:usize> From<[T;N]>              for EnhancedVec<T>, via Vec<T>);
// utils::try_into_via!(impl<T, const N:usize> TryInto<[T;N]>           for EnhancedVec<T>, via Vec<T>);
utils::from_via!(    impl<T, const N:usize> From<EnhancedArray<T,N>> for EnhancedVec<T>, via [T;N]);

impl<T, const N:usize> TryInto<[T;N]> for EnhancedVec<T> {
    type Error=Self;
    fn try_into(self) -> Result<[T;N],Self> {
        self.into_inner()
            .try_into()
            .map_err(|v:Vec<T>|v.into())
    }
}