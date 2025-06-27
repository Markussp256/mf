use std::ops::{Add, Mul, Sub};

use algebra_traits::{
    Torsor, ConstElement, Origin, RealNumber, ScalarMul, TryDiv, Vectorspace1d
};

use num_traits::{Inv, Pow, Zero};

use crate::{Point3, Vector3, primitives::UnitVector};

use phys_units::generic::Angle;

use super::{Rotation3Vector, Translation};

use std::fmt::Debug;
// rotation in 3-dimensional space


// note that rotation can encode not only from original position to final position but also movement 
// for example we can have multiple rotations

#[derive(Clone, Debug)]
pub struct Rotation3Point<F,A> {
    point_on_axis:Point3<A>,
    rot3vector:Rotation3Vector<F>,
}

impl<F, A> Rotation3Point<F, A> {
    pub fn new(point_on_axis:Point3<A>, rot3vector:Rotation3Vector<F>) -> Self {
        Self{point_on_axis, rot3vector}
    }

    pub fn identity() -> Self where A:ConstElement, F:Zero {
        Self{point_on_axis:Point3::ELEMENT,
             rot3vector:Rotation3Vector::zero()}
    }

    pub fn is_identity(&self) -> bool where F:Zero {
        self.rot3vector.is_identity()
    }

    pub fn get_point_on_axis(&self) -> &Point3<A> {
        &self.point_on_axis
    }

    pub fn rot3vector(&self) -> &Rotation3Vector<F> {
        &self.rot3vector
    }
}

impl<F:std::ops::Neg<Output=F>, A> Inv for Rotation3Point<F, A> {
    type Output=Self;

    fn inv(self) -> Self::Output {
        Self{point_on_axis:self.point_on_axis,
             rot3vector:self.rot3vector.inv()}
    }
}

impl<F:RealNumber, A:Sub<Output=V>, V:TryDiv<Output=F>> Rotation3Point<F, A> {

    pub fn abs_angle(&self) -> Angle<F> {
        self.rot3vector
            .abs_angle()
    }

    pub fn try_into_axis_dir(self) -> Option<UnitVector<F, 3>> {
        self.rot3vector
            .try_into_axis_dir()
    }
}

impl<F:RealNumber,
     V:TryDiv<Output = F>+Vectorspace1d,
     A:Sub<Output=V>+Torsor> Rotation3Point<F, A> {
    pub fn into_parameters_as_angles(self) -> [Angle<F>;3] {
        self.rot3vector
            .into_parameters_as_angles()
    }
}

impl<F:RealNumber, V:ScalarMul<F>+TryDiv<Output=F>, A:Sub<Output=V>> Pow<F> for Rotation3Point<F,A> {
    type Output=Self;
    fn pow(self, f:F) -> Self {
        Self{point_on_axis:self.point_on_axis,
             rot3vector:self.rot3vector.pow(f)}
    }
}


impl<A : Clone+Sub<Output=V>,
     V : 'static+Clone+Vectorspace1d+TryDiv<Output=F>,
     F : Clone+Mul<V, Output=V>+RealNumber> Rotation3Point<F, A> {
    // split into the Rotaton around origin and a translation
    pub fn split(&self) -> (Rotation3Vector<F>, Translation<Vector3<V>>) where Point3<A> : Origin {
        let diff:Vector3<V>=self.point_on_axis.clone()-Point3::<A>::origin();
        let t=diff.clone()-self.rot3vector.clone()*diff.clone();
        (self.rot3vector.clone(), Translation::new(t))
    }
}


impl<A:Sub<Output=V>+Clone+Add<V, Output=A>, V:TryDiv<Output=F>, F:Mul<V, Output=V>+RealNumber> Mul<Point3<A>> for Rotation3Point<F, A>
where Rotation3Vector<F> : Mul<Vector3<V>, Output=Vector3<V>> {
    type Output=Point3<A>;

    fn mul(self, rhs: Point3<A>) -> Self::Output {
        let poa=self.point_on_axis;
        poa.clone()+self.rot3vector*(rhs-poa)
    }
}