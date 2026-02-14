use num_traits::{Zero,One};

use matrix_traits::{row_col::ColVectorTryConstruct, Matrix, AlgebraMatrix, MatrixConstructError, MatrixViewSquare, MatrixSquareTryConstruct, Transpose};
use algebra_traits::{basic::Inv, TryInv, RealNumber};
use super::{Orthogonal, SpecialOrthogonal};
use container_traits::{TryFromFn, Get, IntoInner, Iter, IntoSum, TryFromSuperContainer};

type U2=(usize,usize);

#[derive(Clone, Debug, PartialEq,
    algebra_derive::Conjugate,
    algebra_derive::Inv,
    container_derive::IntoInner,
    container_derive::JustContainer,
    container_derive::NewUnchecked,
    derive_more::AsRef,
    derive_more::Index,
    matrix_derive::Identity,
    matrix_derive::Inherit,
    matrix_derive::MatrixNormal,
    matrix_derive::ClosedMatrixMatrixProduct,
    matrix_derive::MatrixShape)]
pub struct Homogeneous<M:MatrixViewSquare>(M) where M::T : RealNumber;

impl<M:Matrix+MatrixViewSquare> Homogeneous<M> where M::T : RealNumber {

    pub fn n(&self) -> usize { self.0.n() }

    pub fn try_into_rot<M2:MatrixSquareTryConstruct<T=M::T>+AlgebraMatrix+TryFromSuperContainer<U2,Self>>(self) -> Result<SpecialOrthogonal<M2>,MatrixConstructError> {
        let n=self.n();
        let m=M2::try_from_super(self,(0,0),(n-1,n-1))?;
        let o=Orthogonal::try_new(m).unwrap();
        let so=SpecialOrthogonal::try_new(o,M::T::one()).ok().unwrap();
        Ok(so)
    }

    pub fn translation_values(& self) -> impl ExactSizeIterator<Item=M::T> {
        let n=self.n();
        let col:Vec<M::T>=
            self.0
                .try_col_view(n-1).unwrap()
                .iter()
                .cloned()
                .take(n-1)
                .collect();
        col.into_iter()
    }

    pub fn into_parts<M2 : MatrixSquareTryConstruct<T=M::T>+AlgebraMatrix+TryFromSuperContainer<U2,Self>,
                      C  : ColVectorTryConstruct<T=M::T>>(self) -> Option<(SpecialOrthogonal<M2>, C)> where M::T : Clone {
        let t=C::any_from_iter(None,self.translation_values()).ok()?;
        let so=self.try_into_rot().ok()?;
        Some((so,t))
    }
}


impl<M:MatrixViewSquare> TryFrom<SpecialOrthogonal<M>> for Homogeneous<M> where M::T : RealNumber {
    type Error=SpecialOrthogonal<M>;
    fn try_from(m:SpecialOrthogonal<M>) -> Result<Self,SpecialOrthogonal<M>> {
        let i=m.n()-1;
        for j in 0..(m.n()-1) {
            if m.get((i,j)).unwrap().is_zero() {
                return Err(m);
            }
        }
        if m.get((i,i)).unwrap() != &M::T::one() {
            return Err(m);
        }
        Ok(Homogeneous(m.into_inner()))
    }
}

impl<M:MatrixViewSquare+Transpose<Output=M>> Transpose for Homogeneous<M> where M::T : RealNumber {
    type Output=SpecialOrthogonal<M>;

    fn into_transpose(self) -> Self::Output {
        SpecialOrthogonal::<M>::from(self)
            .into_transpose()
    }
}

impl<M:MatrixViewSquare+TryFromFn<U2,M::T>> Inv for Homogeneous<M> where M::T : Clone+RealNumber {
    type Output=Self;
    fn inv(self) -> Self {
        let n=self.n();
        Self(<M as TryFromFn<U2,M::T>>::try_from_fn(
            (n,n),
            |(i,j)|{
                if i < n-1 && j < n-1 {
                    self.get((j,i)).unwrap().clone()
                } else if i == n-1 {
                    if j == n-1 {
                        M::T::one()
                    } else {
                        M::T::zero()
                    }
                }
                else {
                    -(0..(n-1))
                        .into_iter()
                        .map(|k|self.get((k,i)).unwrap().clone()*
                                       self.get((k,n-1)).unwrap().clone())
                            .into_sum()
                }
            }).ok().unwrap())
    }
}

impl<M:MatrixViewSquare> TryInv for Homogeneous<M> where Self : Inv<Output=Self>, M::T : RealNumber {
    type Output=Self;
    type Error=();
    fn is_invertible(&self) -> Result<(),()> { Ok(()) }
    fn try_inv(self) -> Result<Self,()> {
        Ok(self.inv())
    }
}