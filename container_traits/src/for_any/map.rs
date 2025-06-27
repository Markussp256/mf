pub trait Map<TIn,TOut> {
    type Output;
    fn map(self, f:impl Fn(TIn) -> TOut) -> Self::Output;
}

pub trait ClosedMap<F> : Map<F,F,Output=Self> {}
impl<F,A:Map<F,F,Output=A>> ClosedMap<F> for A {}

impl<F,FOut> Map<F,FOut> for Vec<F> {
    type Output = Vec<FOut>;
    fn map(self,f:impl Fn(F) -> FOut) -> Vec<FOut> {
        self.into_iter()
            .map(f)
            .collect()
    }
}

impl<F, FOut, const N:usize> Map<F,FOut> for [F;N] {
    type Output=[FOut;N];
    fn map(self,f:impl Fn(F) -> FOut) -> [FOut; N] {
        self.map(f)
    }
}
