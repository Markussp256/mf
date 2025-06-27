
// use std::ops::Div;

// use algebra_traits::{Affinespace, Max, MetricAffineSpace1d, Norm, Normalize, NormedVectorspace1d, Scalar, Vectorspace1d};
// use num_traits::Zero;

// use algebra::AffineSubSpaceN;
// use super::{point::Point, UnitVector, Vector};

// pub type Line<F:Scalar, A, const N:usize>=AffineSubSpaceN<F, Point<A,N>, 1>;


// impl<F:Clone+Scalar,
//      A:Clone+MetricAffineSpace1d<F, V=V>,
//      V:Clone+NormedVectorspace1d,
//      TR,
//      const N:usize> Line<F, A, N> where 
//      Vector<V,N> : Normalize<DivOutput=Vector<TR,N>>,
//      Vector<TR, N> : TryInto<UnitVector<TR,N>> {
//     pub fn direction(&self) -> UnitVector<TR, N> {
//         let basis=self.basis();
//         (basis[1].clone()-basis[0].clone()).try_dir().unwrap()
//     }
// }



// #[derive(Clone, Debug)]
// pub struct Line<A:Affinespace>(AffineSubSpace<A>);

// impl<A:Clone+Affinespace> Line<A> {
//     pub fn from_point_vector(p:A,v:A::V) -> Self {
//         Self{p,v}
//     }

//     pub fn try_from_two_points(p0:A, p1:A) -> Option<Self> {

//     }


//     pub fn point(&self) -> A {
//         self.p.clone()
//     } 

//     pub fn direction(&self) -> &A::V {
//         &self.v
//     }

//     pub fn proj(&self, p:A) -> A where A:Copy, A::V:Copy+Scalarproduct<f64> {
//         self.p+ (p-self.p).scalar_product(self.v)*self.v/self.v.norm_squared()
//     }

//     pub fn distance(&self, p:A) -> f64 where A:Copy, A::V:Copy+Scalarproduct<f64> {
//         (p-self.proj(p)).norm()
//     }
// }
// use core::fmt::Debug;
// use nalgebra::{Point3, Vector3};
// use std::ops::Div;

// pub struct Line<T: Copy + PartialEq + Debug + 'static, T2: Div<T>> {
//     p: Point3<T>,
//     v: Vector3<T2>,
// }
