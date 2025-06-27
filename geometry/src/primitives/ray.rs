
use super::{point::Point, UnitVector};

#[derive(Clone, Debug)]
pub struct RayGeneric<F, A, const N:usize> {
    p:Point<A, N>,
    v:UnitVector<F, N>
}

pub type Ray=RayGeneric<f64,f64,3>;

impl<F, A, const N:usize> RayGeneric<F, A, N> {

    pub fn new(p:Point<A,N>, v:UnitVector<F, N>) -> Self {
        Self{p, v}
    }

    pub fn point(&self) -> &Point<A,N> {
        &self.p
    } 

    pub fn direction(&self) -> &UnitVector<F,N> {
        &self.v
    }

    pub fn into_point_direction(self) -> (Point<A,N>, UnitVector<F,N>) {
        (self.p, self.v)
    }
}




// use core::fmt::Debug;
// use nalgebra::{Point3, Vector3};
// use std::ops::Div;

// pub struct Line<T: Copy + PartialEq + Debug + 'static, T2: Div<T>> {
//     p: Point3<T>,
//     v: Vector3<T2>,
// }
