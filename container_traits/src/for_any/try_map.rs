use crate::LinearContainerConstructError;

pub trait TryMap<TIn,TOut,E> {
    type Output;
    fn try_map(self, f:impl Fn(TIn) -> TOut) -> Result<Self::Output,E>;
}

pub trait TryClosedMap<F,E> : TryMap<F,F,E,Output=Self> {}
impl<F, E, A:TryMap<F,F, E, Output=A>> TryClosedMap<F,E> for A {}

// error is infallible but we will need LCCE
impl<F,FOut> TryMap<F,FOut,LinearContainerConstructError> for Vec<F> {
    type Output = Vec<FOut>;
    fn try_map(self,f:impl Fn(F) -> FOut) -> Result<Vec<FOut>,LinearContainerConstructError> {
        Ok(self.into_iter()
               .map(f)
               .collect())
    }
}

impl<F, FOut, const N:usize> TryMap<F,FOut, LinearContainerConstructError> for [F;N] {
    type Output=[FOut;N];
    fn try_map(self,f:impl Fn(F) -> FOut) -> Result<[FOut; N],LinearContainerConstructError> {
        Ok(self.map(f))
    }
}