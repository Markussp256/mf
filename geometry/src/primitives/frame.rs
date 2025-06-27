use algebra_traits::{Origin, RealNumber, Torsor};
use num_traits::{Inv, One, Zero};
use utils::IntoThis;
use super::{ray::RayGeneric, Point, Vector};
use crate::{trafos::{Translation, SE}, UnitVector};
use std::ops::{Sub, Div, Mul};

use matrix_wrappers::{OrthogonalMatrix, SpecialOrthogonalMatrix};
use matrix_traits::{Matrix, MatrixTryConstruct, matrix_operations::identity::for_static::Identity};

#[derive(Clone, Debug)]
pub struct FrameGeneric<F:RealNumber, A:'static, const N:usize> {
    position:Point<A, N>,
    orientation:OrthogonalMatrix<F, N>
}

pub type Frame=FrameGeneric<f64, f64, 3>;

impl<F:'static+Clone+RealNumber+Zero+One, A:Origin, const N:usize> Default for FrameGeneric<F, A, N> {
    fn default() -> Self {
        Self{position: Point::<A,N>::origin(),
             orientation: OrthogonalMatrix::identity()}
    }
}

impl<F:RealNumber,
     A:Sub<Output=F>+Torsor,
     const N:usize> FrameGeneric<F, A, N> {

    pub fn new(position:Point<A, N>, orientation:OrthogonalMatrix<F, N>) -> Self {
        Self{position, orientation}
    }

    pub fn try_new(position:Point<A,N>,
                   vectors:[Vector<F,N>;N]) -> Option<Self> {
        let vectors:[algebra::Vector<F,N>;N]=vectors.map(|v|v.into());
        OrthogonalMatrix::<F,N>::try_from_cols(vectors.into_iter().map(|v|v.into()))
            .map(|o|Self::new(position, o))
            .ok()
    }

    pub fn position(&self) -> &Point<A,N> {
        &self.position
    }

    pub fn orientation(&self) -> &OrthogonalMatrix<F, N> {
        &self.orientation
    }

    pub fn into_parts(self) -> (Point<A,N>,OrthogonalMatrix<F,N>) {
        (self.position, self.orientation)
    }

    pub fn into_point_vectors(self) -> (Point<A,N>, [Vector<F,N>;N]) {
        let (position, orientation)=self.into_parts();
        let vectors=
            utils::iter::next_chunk(& mut
            orientation
                .into_cols()
                .map(|c|c.into_this::<algebra::Vector<F,N>>().into()))
                .ok().unwrap();
        (position, vectors)
    }

    pub fn xray(self) -> RayGeneric<F, A, N> {
        let (position,orientation)=self.into_parts();
        let v:crate::Vector<F,N>=
            orientation
                .col(0).unwrap()
                .into_this::<algebra::Vector<F,N>>()
                .into_this::<crate::Vector<F,N>>();
        let uv=UnitVector::<F,N>::try_new(v)
                .ok().unwrap();
        RayGeneric::new(position, uv)
    }

}


impl<F:RealNumber,
     A:Sub<Output=F>+Torsor+Clone,
     const N:usize> Mul<FrameGeneric<F, A, N>> for SE<F,F,N> where Point<A, N> : Origin {
        type Output = FrameGeneric<F, A, N>;
        fn mul(self, rhs:FrameGeneric<F, A, N>) -> FrameGeneric<F, A, N> {
            let (pt, vectors)=rhs.into_point_vectors();
            let pt: Point<A, N>=self.clone()*pt;
            let vectors=vectors.map(|v|self.clone()*v);
            Self::Output::try_new(pt, vectors).unwrap()
        }
}

impl<F:RealNumber,
     A:Sub<Output=F>+Torsor+Clone,
     const N:usize> FrameGeneric<F, A, N>
    where Point<A, N> : Origin {
    pub fn from_to(from:Self, to:Self) -> SE<F, F, N> {
        use matrix_traits::MatrixMatrixProduct;
        let from_m=from.orientation().clone();
        let to_m=to.orientation().clone();
        let rot=to_m.matrix_matrix_product(from_m.inv());
        let rot=SpecialOrthogonalMatrix::try_new(rot,F::one()).ok().unwrap();
        let rot_se=SE::from_parts(rot,Translation::<Vector<F, N>>::zero());
        let t=Translation::<Vector<F,N>>::from_to(rot_se.clone()*from.position().clone(), to.position().clone());
        <SE::<F, F, N> as Mul>::mul(t.into(), rot_se)
    }
}

#[test]
fn test_from_to() {
    use crate::Point3;
    let from=Frame::default();
    let p=crate::Point3::new(1.2, 3.2, 2.4);
    let o3=
    OrthogonalMatrix::try_new(matrix::matrix![1.0, 0.0, 0.0;0.0, 0.8, 0.6; 0.0, -0.6, 0.8]).unwrap();
    let so3=SpecialOrthogonalMatrix::try_new(o3.clone(),1.0).unwrap();

    let to=Frame::new(p, o3);
    let se3=Frame::from_to(from, to);
    assert_eq!(se3.rot(), &so3);
    assert_eq!(se3.t().vector(), &(p-Point3::<f64>::origin()))
}

impl<F:RealNumber,
     A:Sub<Output=F>+Torsor+Clone,
     const N:usize> Div for FrameGeneric<F, A, N> where Point<A, N> : Origin {
    type Output=SE<F, F, N>;
    fn div(self, rhs:Self) -> Self::Output {
        Self::from_to(rhs, self)       
    }
}