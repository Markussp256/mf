use std::ops::Mul;
use algebra_traits::TryAdd;
use super::Manifold;

pub trait TangentBundle<F,E,M: Manifold<F,E>>
where
    Self: Sized
         +TryAdd<Output = Self>
         +Mul<F, Output = Self>,
    E : From<<Self as TryAdd>::Error>
{
    fn proj_m(self) -> M;
    fn zero(pt: M) -> Self;

    fn try_sum(pt: M, iter: impl Iterator<Item = Self>) -> Result<Self, E> {
        let mut res=Self::zero(pt);
        for i in iter {
            res=res.try_add(i)?;
        }
        Ok(res)
    }
}