use container_traits::{ClosedMap, ContainerTryConstruct, LinearContainerConstructError, Map};
use crate::{ClosedTryDiv, Distance, DivError, Norm, NormSquared, RealNumber, Scalar, TryDiv, TryNormalize};

use super::{InnerProductSpace1d, Scalarproduct, TryInnerProductSpace};

pub trait ScalarVector<E=LinearContainerConstructError>
  :  ContainerTryConstruct<usize,E>
    +TryInnerProductSpace<
    Self::T,
      NormT      = <Self::T as Scalar>::RealType,
      Norm2T     = <Self::T as Scalar>::RealType,
      TryDistT   = <Self::T as Scalar>::RealType,
      TryScProdT =  Self::T>
    +TryDiv <Self::T,Output=Self,Error=DivError>
    +TryDiv<<Self::T as Scalar>::RealType,Output=Self,Error=DivError>
    +TryNormalize
    +ClosedMap<Self::T> where Self::T : Scalar {}

impl<F : Scalar<RealType=R>,
     R : RealNumber,
     E,
     C : ContainerTryConstruct<usize,E,T=F>
        +TryInnerProductSpace<
          F,
          NormT      = R,
          Norm2T     = R,
          TryDistT   = R,
          TryScProdT = F>
        +TryDiv<F,Output=C,Error=DivError>
        +TryDiv<R,Output=C,Error=DivError>
        +TryNormalize
        +ClosedMap<F>> ScalarVector<E> for C {}

pub trait Vector<E=LinearContainerConstructError>
  :  ContainerTryConstruct<usize,E>
    +TryInnerProductSpace<
      Self::F,
      NormT      = <Self::T as Norm>::NormT,
      Norm2T     = <Self::T as NormSquared>::Norm2T,
      TryDistT   = <Self::T as Distance>::DistT,
      TryScProdT = <Self::T as Scalarproduct>::ScProdT>
    +TryDiv <Self::T,Output=Self::ScalarVector,Error=DivError>
    +TryDiv <Self::F,Output=Self,Error=DivError>
    +TryDiv<<Self::F as Scalar>::RealType,Output=Self,Error=DivError>
    +Map<Self::T,Self::F,Output=Self::ScalarVector>
    where Self::T : InnerProductSpace1d+TryDiv<Output=Self::F> {
  type F : Scalar+ClosedTryDiv<Error=DivError>;
  type ScalarVector : ScalarVector<E,T=Self::F>;
}


impl<V : InnerProductSpace1d+TryDiv<Output=F>,
     F : Scalar<RealType = R>,
     R : RealNumber,
     E,
     C : ContainerTryConstruct<usize,E, T=V>
        +TryInnerProductSpace<
          F,
          NormT      = V::NormT,
          Norm2T     = V::Norm2T,
          TryDistT   = V::DistT,
          TryScProdT = V::ScProdT>
        +TryDiv <V,Output=C2,Error=DivError>
        +TryDiv <F,Output=C, Error=DivError>
        +TryDiv <R,Output=C, Error=DivError>
        +Map<V,F,Output=C2>,
    C2 : ScalarVector<E,T=F>> Vector<E> for C {
  type F=F;
  type ScalarVector = C2;
}

// pub trait Vector : GenericVector<<Self as Vector>::V,
//                                  <Self as Vector>::F> {
//   type V:InnerProductSpace1d+TryDiv<Output=<Self as Vector>::F>;
//   type F:Scalar;
// }

// impl<V : InnerProductSpace1d+TryDiv<Output=<Self as Vector>::F>,
//      F : Scalar,
//      C : GenericVector<V,F>> Vector for C {
//   type V=V;
//   type F=F;
// }