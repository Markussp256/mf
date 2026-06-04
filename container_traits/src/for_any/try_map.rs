use crate::LinearContainerConstructError as LCCE;
use crate::Map;
use generic_array::{ArrayLength, GenericArray};


pub trait TryMap<TIn,TOut,E> {
    type Output;
    fn try_map(self, f:impl Fn(TIn) -> TOut) -> Result<Self::Output,E>;
}

pub trait TryClosedMap<F,E> : TryMap<F,F,E,Output=Self> {}
impl<F, E, A:TryMap<F,F, E, Output=A>> TryClosedMap<F,E> for A {}

macro_rules! impl_try_map {
    () => {
        fn try_map(self,f:impl Fn(F) -> FOut) -> Result<<Self as TryMap<F,FOut,LCCE>>::Output,LCCE> {
            Ok(self.map(f))
        }
    };
}

// error is infallible but we will need LCCE
impl<F,FOut> TryMap<F,FOut,LCCE> for Vec<F> {
    type Output = Vec<FOut>;
    impl_try_map!();
}

impl<F,FOut,N:ArrayLength> TryMap<F,FOut,LCCE> for GenericArray<F,N> {
    type Output = GenericArray<FOut,N>;
    impl_try_map!();
}

impl<F, FOut, const N:usize> TryMap<F,FOut, LCCE> for [F;N] {
    type Output=[FOut;N];
    impl_try_map!();
}