
use super::TryDiv;

impl<T  : TryDiv<T2,Output = TR, Error=E>,E,
     T2 : Clone, TR> TryDiv<T2> for Vec<T> {
    type Output=Vec<TR>;
    type Error=E;
    fn is_divable_by(&self,rhs:&T2) -> Result<(),E> {
        self.iter()
            .map(|ti|ti.is_divable_by(rhs))
            .collect::<Result<Vec<_>,E>>()
            .map(|_|())
    }

    fn try_div(self,rhs:T2) -> Result<Self::Output,E> {
        self.into_iter()
            .map(|ti|ti.try_div(rhs.clone()))
            .collect()
    }
}

impl<T  : TryDiv<T2,Output = TR, Error=E>,E,
     T2 : Clone, TR,
     const N:usize> TryDiv<T2> for [T;N] {
    type Output=[TR;N];
    type Error=E;

    fn is_divable_by(&self,rhs:&T2) -> Result<(),E> {
        self.iter()
            .map(|ti|ti.is_divable_by(rhs))
            .collect::<Result<Vec<_>,E>>()
            .map(|_|())
    }

    fn try_div(self,rhs:T2) -> Result<Self::Output,E> {
        self.into_iter()
            .map(|ti|ti.try_div(rhs.clone()))
            .collect::<Result<Vec<TR>,E>>()
            .map(|v|v.try_into().ok().unwrap())
    }
}