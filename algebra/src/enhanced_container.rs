use std::ops::{Div, Index, IndexMut, Mul, Neg};
use std::fmt::Debug;
use algebra_traits::NonZero;
use container_traits::{AsMutSlice, AsSlice, ClosedMap, ContainerConstructError, ContainerTryConstruct, IntoInner, ItemT, Map};
use derive_more::{AsRef, From};

// all own traits
use algebra_derive::*;

#[derive(
    Clone, Debug, PartialEq,
    AsRef, ConstElement, From,
    container_derive::Container,
    Basis, Conjugate, Crossproduct, Distance, IsAZero, Norm, NormSquared,
    ScalarDiv, TryScalarDiv, ScalarMul, Scalarproduct,
    TryAdd, TryDiv, TryNormalize, TrySub
)]
pub struct EnhancedContainer<C>(C);

// GenericArray does not implement Index so we need to impl it here
impl<C:AsSlice<T>+ItemT<T=T>,T> Index<usize> for EnhancedContainer<C> {
    type Output=T;
    fn index(&self, index: usize) -> &Self::Output {
        &self.as_slice()[index]
    }
}

impl<C:AsMutSlice<T>+ItemT<T=T>,T> IndexMut<usize> for EnhancedContainer<C> where Self : Index<usize,Output=T> {
    fn index_mut(&mut self, index: usize) -> & mut Self::Output {
        &mut self.as_mut_slice()[index]
    }
}

impl<C> EnhancedContainer<C> {
    pub const fn new(c:C) -> Self {
        Self(c)
    }
}
pub trait IntoEnhancedContainer {
    type OutputC;
    fn into_enhanced_container(self) -> EnhancedContainer<Self::OutputC>;
}

impl<C : algebra_traits::operators::basic::Neg<Output=COut>,COut> Neg for EnhancedContainer<C> {
    type Output = EnhancedContainer<COut>;
    fn neg(self) -> Self::Output {
        EnhancedContainer::new(<C as algebra_traits::operators::basic::Neg>::neg(self.into_inner()))
    }
}

impl<C :ContainerTryConstruct<usize,ContainerConstructError<usize>,T=F>+Map<F,FR,Output=C2>,
     C2:'static+ContainerTryConstruct<usize,ContainerConstructError<usize>,T=FR>,
     F :Mul<F2,Output=FR>,
     F2:Clone,
     FR> Mul<F2> for EnhancedContainer<C> {
    type Output=EnhancedContainer<C2>;
    fn mul(self, rhs: F2) -> Self::Output {
        EnhancedContainer::new(self.0.map(|x:F|x*rhs.clone()))
    }
}

impl<C :ContainerTryConstruct<usize,ContainerConstructError<usize>,T=F>+Map<F,FR,Output=C2>,
     C2:'static+ContainerTryConstruct<usize,ContainerConstructError<usize>,T=FR>,
     F :algebra_traits::TryDiv<F2,Output=FR,Error=E>,
     E : Debug,
     F2:Clone,
     FR> Div<NonZero<F2>> for EnhancedContainer<C> {
    type Output=EnhancedContainer<C2>;
    fn div(self, rhs: NonZero<F2>) -> Self::Output {
        EnhancedContainer::new(self.0.map(|x:F|x.try_div(rhs.as_ref().clone()).unwrap()))
    }
}

use algebra_traits::div_by_small_natural::*;

macro_rules! impl_divi {
    ($i:literal) => {
        paste::paste!(
        impl<C : ContainerTryConstruct<usize,ContainerConstructError<usize>,T=F>+ClosedMap<F>,
             F : [<Div $i>]>
             [<Div $i>] for EnhancedContainer<C> {
            fn [<div $i>](self) -> Self {
                Self(self.0.map(|x|x.[<div $i>]()))
            }
        });
    };
}
impl_divi!(2);
impl_divi!(3);
impl_divi!(4);
impl_divi!(5);
impl_divi!(6);
impl_divi!(7);
impl_divi!(8);
impl_divi!(9);
impl_divi!(10);
