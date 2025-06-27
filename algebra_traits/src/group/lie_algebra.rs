// use alga::linear::VectorSpace;

use crate::{Scalar, Vectorspace};
use std::ops::Mul;

pub trait LieAlgebra<F:Scalar+Mul<Self, Output=Self>> : Vectorspace<F> {
    fn lie_bracket(x: Self, y: Self) -> Self;
}

// example
// #[derive(Clone, PartialEq, Debug)]
// pub struct My<T>(T);

// crate::impl_Zero_ !(My);
// crate::impl_gen_additive!(My);
// crate::impl_gen_multiplicative!(My);
// type Myf64 = My<f64>;
// impl AdditiveGroup for Myf64 {}

// crate::impl_vectorspace!(f64,Myf64);
// impl LieAlgebra<f64> for Myf64 {
//     fn lie_bracket(x:Self,y:Self) -> Self {
//         x*y-y*x
//     }
// }
