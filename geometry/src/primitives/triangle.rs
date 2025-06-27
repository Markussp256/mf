use std::ops::Sub;

use algebra_traits::{ConstNonZero, Crossproduct, TryNormalize, Scalar, Torsor, TryDiv, Vectorspace1d};
use crate::{Point, UnitVector};



pub struct Triangle<A:'static, const N:usize>([Point<A, N>;3]);


impl<A, const N:usize> Triangle<A,N> {
    pub fn new(pts:[Point<A,N>;3]) -> Self {
        Self(pts)
    }
}

impl<F:Clone+Scalar, A:Clone+Sub<Output=V>+Torsor, V:Clone+TryDiv<Output=F>+Vectorspace1d> Triangle<A,3> {
    pub fn normal(self) -> Option<UnitVector<F, 3>> {
        let [u0,u1,u2]=self.0;
        let d10=(u1-u0.clone()).try_div(<V as ConstNonZero>::NONZERO).unwrap();
        let d20=(u2-u0).try_div(<V as ConstNonZero>::NONZERO).unwrap();
        let cp=d10.cross_product(d20);
        UnitVector::try_new(
            cp.try_normalize().ok()?.1).ok()
    }
}