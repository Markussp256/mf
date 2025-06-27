use std::ops::Sub;

use algebra_traits::{InnerProductSpace, Origin, Scalar, Scalarproduct, Torsor};
use container_traits::for_static::NumberOfDegreesOfFreedom;

pub trait Submanifold<F:Scalar> : Sized+NumberOfDegreesOfFreedom<F> {

    type AmbientSpace : Origin+Torsor+NumberOfDegreesOfFreedom<F>+Sub<Output=Self::V>;
    type V:InnerProductSpace<F>;


    // required methods
    fn embedding(self) -> Self::AmbientSpace;

    // submanifold will be defined as those elements that yield zero
    // fn implicit_equations(e:Self::AmbientSpace) -> Vec<f64>;

    fn project(e:Self::AmbientSpace) -> Self;

    // self.tangent_space(0) is supposed to yield the embeding of self
    fn tangent_space(self, vs:Vec<F>) -> Self::AmbientSpace;

    // application of second derivative of the projection onto a tangent vector
    fn d2project(e:Self::AmbientSpace, v:Self::V) -> Self::V;

    // provided methods

    // needs to be subtracted from the hessian for the riemannian hessian method
    fn riemannian_hessian_correction(self, grad:impl Fn(&Self::V) -> Self::V) -> <Self::V as Scalarproduct>::ScProdT {
        // ortsvector
        let ov:Self::V=self.embedding()-Self::AmbientSpace::origin();

        grad(&ov).scalar_product(ov)
    }
}