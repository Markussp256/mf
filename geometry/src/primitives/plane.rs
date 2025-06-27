
// all points p that satisfy <p,dir>=val

use std::ops::Sub;

use algebra_traits::{Torsor, Scalar, TryDiv, Vectorspace1d};
use super::{Point, UnitVector};

use container_traits::{IntoInner, Map};

pub struct Plane<F:'static, A:'static, const N:usize> {
    pt:Point<A,N>,
    normal:UnitVector<F, N>,
}

impl<F, A, const N:usize> Plane<F, A, N> {
    pub fn new(pt:Point<A, N>, normal:UnitVector<F, N>) -> Self {
        Self{pt, normal}
    }

    pub fn into_parts(self) -> (Point<A,N>, UnitVector<F,N>) {
        (self.pt,self.normal)
    }
}


impl<F:Scalar,
     V:TryDiv<Output=F>+Vectorspace1d,
     A:Sub<Output=V>+Torsor, const N:usize> Plane<F, A, N> {

    pub fn signed_distance(self, p:Point<A,N>) -> V {
        let (pt,normal)=self.into_parts();
        V::linear_combination(normal
                                    .into_iter()
                                    .zip((p-pt).into_iter()))
    }

    pub fn proj(self, p:Point<A,N>) -> Point<A,N> where Self:Clone, A:Clone, V:Clone {
        let sd=self.clone().signed_distance(p.clone());
        p-self.normal
              .into_inner()
              .map(|fi|sd.clone().scalar_mul(&fi))
    }

}