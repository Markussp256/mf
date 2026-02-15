// basis of a finite-dimensional Vectorspace

use std::ops::Sub;

use algebra::EnhancedArray;
use algebra_traits::{ConstElement, FiniteDimensionalInnerProductSpace, Scalar, Tolerance, Torsor};

use container_traits::{Inner, Parameter};

use num_traits::Zero;
use super::{AffineCoordinates, AffineSubSpace,AffineSubSpaceDyn};

#[derive(Clone, Debug)]
pub struct AffineBasis<F, A, const DIM:usize>(AffineSubSpace<F, A, DIM>);

impl<F, V:FiniteDimensionalInnerProductSpace<F,DIM>, A: Sub<Output=V>+Torsor+ConstElement, const DIM:usize> AffineBasis<F, A, DIM> {
    pub fn basis(&self) -> &[A;DIM] {
        self.0
            .basis()
            .inner()
    }
}

impl<F, V:FiniteDimensionalInnerProductSpace<F,DIM>, A: Sub<Output=V>+Torsor+ConstElement, const DIM:usize, S:Into<AffineSubSpace<F,A,DIM>>> From<S> for AffineBasis<F, A, DIM> {
    fn from(value: S) -> Self {
        Self(value.into())
    }
}

impl<F,
     V : FiniteDimensionalInnerProductSpace<F,DIM>,
     A : Sub<Output=V>+Torsor+ConstElement, const DIM:usize> TryFrom<AffineSubSpaceDyn<F,A>> for AffineBasis<F, A, DIM> {
    type Error = AffineSubSpaceDyn<F,A>;
    fn try_from(value: AffineSubSpaceDyn<F,A>) -> Result<Self, AffineSubSpaceDyn<F,A>> {
        value.try_into()
             .map(|s|Self(s))
    }
}

impl<F:Clone+Scalar,
     const DIM:usize,
     V : Clone+FiniteDimensionalInnerProductSpace<F,DIM>,
     A : Clone+Sub<Output=V>+Torsor+ConstElement+Tolerance> AffineBasis<F, A, DIM>
    where V::ScProdT : Clone+Zero+Parameter<F>,
          A::DistT : PartialOrd { // +Scalarproduct<Output=SP>+Clone+Distance<SignedOutput = DS>, SP:Zero+Parameters1<F>
    pub fn try_new(vs:Vec<A>) -> Option<(Self,EnhancedArray<usize,DIM>)> {
        let (ss,ind)=AffineSubSpaceDyn::new(vs);
        let ss=ss.try_into().ok()?;
        let ind=ind.try_into().ok()?;
        Some((ss,ind))
    }

    pub fn affine_combination(self, ws:AffineCoordinates<F,DIM>) -> A {
        self.0
            .affine_combination(ws)
    }

    pub fn find_coordinates(self, a:A) -> AffineCoordinates<F,DIM> {
        self.0
            .find_coordinates_of_projection(a)
    }
}
















// use std::ops::Sub;

// use algebra_traits::{
//     ConstElement,
//     InnerProductSpace,
//     Scalar,
//     Tolerance,
//     Torsor
// };


// use num_traits::Zero;

// use super::{AffineSubSpace, AffineCoordinates};

// #[derive(Clone, Debug)]
// pub struct AffineBasis<F, A, const DIMV:usize> {
//     space:AffineSubSpace<F, A>
// }

// impl<F:Scalar,
//      A:Clone+Sub<Output=V>+Torsor+ConstElement+Tolerance,
//      V:Clone+InnerProductSpace<F>,
//      const DIMV:usize> AffineBasis<F, A, DIMV> 
//     where A::DistT : PartialOrd,
//           V::ScProdT : Zero {
//     pub fn affine_combination(&self, ws:AffineCoordinates<F>) -> A {
//         self.clone()
//             .space
//             .affine_combination(ws)
//     }

//     pub fn basis(&self) -> &Vec<A> {
//         self.space
//             .basis()
//     }

//     pub fn try_new(vs:Vec<A>) -> Option<(Self, Vec<usize>)> {
//         let (space, inds)=AffineSubSpace::new(vs);
//         (space.dimension() == DIMV+1).then(||
//             (Self{space},inds))
//     }

//     pub fn find_coordinates(&self, p:A) -> AffineCoordinates<F> {
//          self.space
//              .find_coordinates_of_projection(p)
//     }
// }



// // use super::AffineSubSpaceN;


// // pub type AffineBasis<A:FiniteDimensionalAffinespace>=AffineSubSpaceN<A,{A::DIM}>;


// // #[derive(Debug)]
// // pub struct AffineBasis<A:FiniteDimensionalAffinespace>(AffineSubSpaceN<A,{A::DIM}>);


// // impl<A:FiniteDimensionalAffinespace+Clone> AffineBasis<A> where A::V : Vectorspace<f64>+Parameters {
// //     pub fn try_new(vs:Vec<A>, tol:f64) -> Option<(Self,Vec<usize>)> {
// //         let res=AffineSubSpace::new(vs, tol);
// //         match res.0.dimension() {
// //             A::DIM => Some((Self(res.0),res.1)),
// //             _ => None
// //         }
// //     }

// //     pub fn affine_combination(&self, ac:AffineCoordinates) -> A {
// //         assert_eq!(ac.len(), A::DIM);
// //         self.0.affine_combination(ac)
// //     }

// //     pub fn find_coordinates(&self, v:A) -> AffineCoordinates {
// //         self.0.find_coordinates_of_projection(v)
// //     }
// // }