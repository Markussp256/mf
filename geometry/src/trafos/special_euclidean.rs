
use algebra_traits::{RealNumber, TryDiv, Vectorspace1d};
use container_traits::{ContainerConstructError, DimensionMismatchError, LenNotEqualToRequiredLenError, LenTooSmallError};
use container_traits::{for_static::TryFromParameters,Parameter, IntoParameters, TryFromLocalParameters, IntoLocalParameters};
use matrix_traits::identity::for_static::Identity;
use num_traits::real::Real;
use num_traits::{One, Zero, Inv};
use utils::IntoThis;
use std::ops::{Mul, Sub};
use std::fmt::Display;
use matrix::Matrix;
use matrix_traits::{MatrixConstructError, MatrixMatrixProduct, MatrixVectorProduct};
use matrix_wrappers::{SpecialOrthogonalMatrix, HomogeneousMatrix};
use crate::trafos::{ScrewMotion, Translation};

use num_traits::Pow;
use crate::{Point, Point3, Vector};

use algebra_traits::{
    Torsor,
    Vectorspace
};

use algebra_traits::Origin;

use super::Rotation3Point;

#[derive(
    Clone, Debug, PartialEq, 
    container_derive::IntoParameters
)]
pub struct SE<F:RealNumber, V, const N: usize> {
    rot: SpecialOrthogonalMatrix<F, N>,
    t: Translation<Vector<V, N>>
}

#[derive(Clone, Debug, thiserror::Error)]
pub enum SETryFromParametersError {
    #[error("problems creating rotation matrix {0}")]
    Rotation(#[from] MatrixConstructError),
    #[error("not enough elements to create an SE {0}")]
    LenTooSmall(#[from] LenTooSmallError),
    #[error("not the correct number of elements to create an SE {0}")]
    LenNotEqualToRequiredLen(#[from] LenNotEqualToRequiredLenError)
}

impl<F:Clone+RealNumber, V:Parameter<F>, const N:usize> TryFromParameters<F,SETryFromParametersError> for SE<F,V,N> {
    fn try_take_away<I:Iterator<Item=F>>(iter:& mut I) -> Result<Self, SETryFromParametersError> {
        let rot=SpecialOrthogonalMatrix::try_take_away(iter)?;
        let mut v_iter=std::iter::from_fn(||iter.next().map(V::from_parameter));
        let t=match Translation::try_take_away(& mut v_iter) {
            Ok(t) => t,
            Err(ContainerConstructError::<usize>::DimensionMismatch(DimensionMismatchError::LenTooSmall(e)) )
            => { return Err(SETryFromParametersError::LenTooSmall(e)); },
            _ => { panic!("unexpected error") }
        };
        Ok(Self{rot,t})
    }
    container_traits::try_from_parameters_impl!(F,SETryFromParametersError);
}



impl<F:RealNumber+Display, V:Display, const N:usize> Display for SE<F, V, N> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "RotationMatrix:")?;
        writeln!(f, "{}", self.rot)?;
        writeln!(f, "{}", self.t)
    }
}

impl<F:RealNumber+Mul<V,Output=V>+Clone, V:'static+Clone+Vectorspace<F>, const N: usize> Mul for SE<F, V, N> {
    fn mul(self, rhs: Self) -> Self {
        let (self_mat, selfvector)=self.into_matrix_algebra_vector();
        let (rhs_mat, rhs_vector)=rhs.into_matrix_algebra_vector();
        Self::from_parts(self_mat.clone().matrix_matrix_product(rhs_mat), 
            Translation::from(self_mat.matrix_vector_product(rhs_vector) + selfvector))
    }
    type Output = Self;
}

impl<F:RealNumber, V:Vectorspace<F>, const N: usize> From<SpecialOrthogonalMatrix<F,N>> for SE<F, V, N> {
    fn from(rot: SpecialOrthogonalMatrix<F,N>) -> Self {
        Self {
            rot,
            t: Translation::zero(),
        }
    }
}

impl<F:Clone+RealNumber+Mul<V,Output=V>,
     A:Clone+Origin+Sub<Output=V>,
     V:'static+Clone+TryDiv<Output=F>+Vectorspace1d> From<Rotation3Point<F, A>> for SE<F, <A as Sub>::Output, 3> {
    fn from(value: Rotation3Point<F, A>) -> Self {
        let (rot,trans)=value.split();
        Self::from_parts(rot.into(), trans)
    }
}

impl<F:RealNumber, V, const N:usize> From<Translation<Vector<V, N>>> for SE<F, V, N> {
    fn from(value: Translation<Vector<V, N>>) -> Self {
        Self::from_parts(SpecialOrthogonalMatrix::identity(), value)
    }
}

impl<F:RealNumber+Mul<V,Output=V>+Clone, V:Clone+Vectorspace<F>, const N: usize> Inv for SE<F, V, N> {
    type Output = Self;
    fn inv(self) -> Self {
        let (rot, vector)=self.into_matrix_algebra_vector();
        let rot_inv = rot.inv();
        Self {
            rot: rot_inv.clone(),
            t: Translation::from(-(rot_inv.matrix_vector_product(vector))),
        }
    }
}

impl<F:RealNumber+Mul<V, Output=V>, V:'static+Clone+Vectorspace<F>, const N: usize> SE<F, V, N> {
    // SpecialOrthogonalMatrix that user does not have to include trait MultiplicativeInverse to use inverse
    pub fn inverse(self) -> Self {
        <Self as Inv>::inv(self)
    }

    pub fn identity() -> Self {
        Self::one()
    }
}

#[cfg(feature = "nalgebra_support")]
impl<F:RealNumber+nalgebra::Scalar, V:TryDiv<Output=F>+Vectorspace<F>, const N: usize>
    From<nalgebra::Isometry<V, nalgebra::Rotation<F, N>, N>> for SE<F, V, N>
    where V:nalgebra::Scalar
{
    fn from(value: nalgebra::Isometry<V, nalgebra::Rotation<F, N>, N>) -> Self {
        Self {
            rot: value.rotation.clone().into(),
            t: value.translation.into(),
        }
    }
}

// nalgebra only has Isometry for N=2,3
macro_rules! into_nalgebra {
    ($N:tt) => {
        #[cfg(feature = "nalgebra_support")]
        impl<V:Vectorspace<f64>>
            Into<nalgebra::Isometry<V, nalgebra::Rotation<f64, $N>, $N>> for SE<f64, V, $N>
            where V: nalgebra::Scalar
        {
            fn into(self) -> nalgebra::Isometry<V, nalgebra::Rotation<f64, $N>, $N> {
                nalgebra::Isometry {
                    rotation: self.rot.clone().into(),
                    translation: self.t.into(),
                }
            }
        }
    };
}

into_nalgebra!(2);
into_nalgebra!(3);

macro_rules! from_homo {
    ($N:tt) => {
        impl From<HomogeneousMatrix<f64,{$N+1}>> for SE<f64, f64, $N> {
            fn from(value:HomogeneousMatrix<f64,{$N+1}>) -> Self {
                let (so,t)=value.into_parts().unwrap();
                Self::from_parts(so,
                                 Translation::new(t))
            }
        }
    };
}

from_homo!(2);
from_homo!(3);


pub type SE2<F= f64, V = F> = SE<F, V, 2>;
pub type SE3<F= f64, V = F> = SE<F, V, 3>;

impl<F:RealNumber, V, const N:usize> SE<F, V, N> {
    pub fn from_parts(rot: SpecialOrthogonalMatrix<F,N>, t: Translation<Vector<V,N>>) -> Self {
        Self { rot, t }
    }
}

impl<F:Clone+Mul<V, Output=V>+RealNumber, A:Clone+Origin+Sub<Output=V>, V:'static+Clone+Vectorspace1d+TryDiv<Output=F>> From<ScrewMotion<F, A>> for SE3<F,V> {
    fn from(value: ScrewMotion<F, A>) -> Self {
        let (rotation, parallel_translation)=value.into_parts();
        let (rot3vector, t1)=rotation.split();
        Self::from_parts(rot3vector.try_into().ok().unwrap(), t1+parallel_translation)
    }
}

impl<F:RealNumber, V:Vectorspace<F>, const N: usize> SE<F, V, N> {
    pub fn rot(&self) -> &SpecialOrthogonalMatrix<F,N> {
        &self.rot
    }

    pub fn rot_mat(&self) -> &Matrix<F, N, N> {
        self.rot()
            .as_ref()
    }
}

impl<F:RealNumber, V, const N:usize> SE<F, V, N> {
    pub fn t(&self) -> &Translation<Vector<V,N>> {
        &self.t
    }

    pub fn into_matrix_algebra_vector(self) -> (SpecialOrthogonalMatrix<F,N>, algebra::Vector<V,N>) {
        (self.rot, self.t
                       .into_vector()
                       .into())
    }
}

// impl<F:Scalar, V:Vectorspace<F>> SE<F, V, 3> {
//     pub fn abs_angle(&self) -> phys_units::Angle {
//         Rotation3Vector::from(self.rot()).abs_angle()
//     }
// }

impl<F:Clone+RealNumber+Mul<V, Output=V>, V:Clone+TryDiv<Output=F>+Vectorspace1d> SE<F, V, 3> {
    pub fn rotation_wrt_point<A:Sub<Output=V>+Torsor>(
        rot: SpecialOrthogonalMatrix<F,3>,
        pt: Point3<A>,
    ) -> Self where Point3<A>:Origin {
        let ptmo:algebra::Vector3<V>= (pt - Point3::<A>::origin()).into();
        Self::from_parts(rot.clone(), Translation::from(ptmo.clone() - rot.matrix_vector_product(ptmo))) // - rot * pt.clone()
    }

    pub fn point_on_rot_axis<A:Sub<Output=V>+Torsor+Parameter<F>+Clone>(&self) -> Point3<A> where SE<F, V, 3> : Into<ScrewMotion<F, A>> { //  Point3<A>:Origin, V:Clone+Norm
        self.clone()
            .into_this::<ScrewMotion<F, A>>()
            .rotation()
            .get_point_on_axis()
            .clone()
    }
}


#[test]
#[cfg(feature = "nalgebra_support")]
fn test_new_rotation_wrt_point() {
    use algebra_traits::Parameters;
    let axisangle = vec![0.1, 0.2, 0.3];
    let rot = SpecialOrthogonalMatrix::<f64,3>::try_from_parameters(axisangle).unwrap();
    let pt = Point3::new(1.0, 3.0, 2.0);
    let my = SE3::rotation_wrt_point::<f64>(rot, pt);
    let their: nalgebra::Isometry<f64, nalgebra::Rotation<f64, 3>, 3> =
        nalgebra::Isometry::rotation_wrt_point(
            rot.into(),
            nalgebra::Point3::new(pt[0], pt[1], pt[2]),
        );
    let their_t: SE<f64, f64, 3> = their.into();
    assert!(my
        .parameters()
        .iter()
        .zip(their_t.parameters())
        .all(|(l, r)| (l - r).abs() < 1e-10));
}

// #[test]
// fn point_on_rot_axis() {
//     use phys_units::{Length, Position, Millimeters};
//     let SpecialOrthogonalMatrix3=SpecialOrthogonalMatrix3::from_scaled_axis(Vector3::new(1.0, 0.0, 0.0));
//     let t=  Vector3::new(1.0, 0.0, 0.0).map(Length::from_mm);
//     let se3=SE3::<Length>::from_parts(SpecialOrthogonalMatrix3, t);
//     let dir=se3.try_rot_axis_direction().unwrap();
//     let pt_on_rot_axis=se3.try_get_point_on_rot_axis::<Position>().unwrap();
//     let diff=se3*pt_on_rot_axis-pt_on_rot_axis;
//     assert!(diff.cross_product(&dir).norm() < Length::from_mm(1e-10));
// }


impl<F:RealNumber+Mul<V, Output=V>, V:'static+Clone+Vectorspace<F>, const N: usize> std::ops::Div for SE<F, V, N> {
    type Output = Self;

    fn div(self, rhs: Self) -> Self::Output {
        self * rhs.inv()
    }
}

impl<F:RealNumber+Mul<V, Output=V>, V:'static+Clone+Vectorspace<F>, const N: usize> One for SE<F, V, N> {
    fn one() -> Self {
        Self::from_parts(SpecialOrthogonalMatrix::identity(), Translation::zero())
    }
}

// for the direction only the orthogonal matrix is relevant
impl<F:RealNumber+Mul<V, Output=V>,
     V:Clone+TryDiv<Output=F>+Vectorspace<F>,
     const N: usize> Mul<Vector<V, N>> for SE<F, V, N> {
    type Output = Vector<V,N>;
    fn mul(self, rhs: Vector<V,N>) -> Self::Output {
        let rhs:algebra::Vector<V,N>=rhs.into();
        let (rot, _)=self.into_matrix_algebra_vector();
        rot.matrix_vector_product(rhs).into()
    }
}

impl<A:Sub<Output=V>+Torsor,
     F:RealNumber+Mul<V, Output=V>,
     V:Clone+TryDiv<Output=F>+Vectorspace<F>,
     const N: usize> Mul<Point<A, N>> for SE<F, V, N> where Point<A, N> : Origin {
    type Output = Point<A,N>;
    fn mul(self, rhs: Point<A,N>) -> Self::Output {
        let diff:algebra::Vector<V,N>=(rhs - Point::<A,N>::origin()).into();
        let (rot, vector)=self.into_matrix_algebra_vector();
        Self::Output::origin() + (rot.matrix_vector_product(diff) + vector).into()
    }
}


impl<F:RealNumber, V:Sub> Pow<F> for SE<F, V, 3>
 where Self : Into<ScrewMotion<F, V>>, ScrewMotion<F, V> : Pow<F, Output=ScrewMotion<F,V>>+Into<Self> {
    type Output=Self;
    fn pow(self, t:F) -> Self {
        self.into_this::<ScrewMotion<F,V>>()
            .pow(t)
            .into()
    }
}



impl<F:RealNumber, V:Parameter<F>, const N:usize> container_traits::for_static::NumberOfDegreesOfFreedom<F> for SE<F, V, N> {
    const NDOFS: usize = (N*(N+1))/2;
}

// impl<F:RealNumber+Mul<V, Output=V>,
//      V:Clone+TryDiv<Output=F>+Vectorspace1d+Parameter<F>> IntoLocalParameters<F> for SE<F,V,3> {
//     fn into_local_parameters(self,rhs:Self) -> impl ExactSizeIterator<Item=F> {
//         (rhs/self).into_parameters()
//     }
// }

impl<F:RealNumber+Mul<V, Output=V>,
     V:TryDiv<Output=F>+Vectorspace1d+Parameter<F>> IntoLocalParameters<F> for SE<F, V, 3> {
    container_traits::impl_into_local_parameters_for_multiplicative_group!();
}

impl<F:RealNumber+Mul<V, Output=V>,
     V:TryDiv<Output=F>+Vectorspace1d+Parameter<F> + Clone,
     E> TryFromLocalParameters<F,E> for SE<F, V, 3> where Self : TryFromParameters<F, E> {
    container_traits::impl_try_from_local_parameters_for_multiplicative_group!();
}

impl<F:RealNumber+Mul<V,Output=V>,
     V:TryDiv<Output=F>+Vectorspace1d+Parameter<F>> geometry_traits::Manifold<F,SETryFromParametersError> for SE<F,V,3> {
}

impl<F:RealNumber+Mul<V, Output=V>,
     V:TryDiv<Output=F>+Vectorspace1d+Parameter<F>> geometry_traits::LieGroup<F,SETryFromParametersError> for SE<F, V, 3> {   
}

#[test]
fn so_conv() {
    use algebra_traits::Parameters;
    let vs = vec![0.0, 0.0, 0.0, 3.1, -2.3, 4.1];
    let se3: SE3<f64> = SE3::<f64>::try_from_parameters(vs.clone()).unwrap();
    println!("The SpecialOrthogonalMatrix3 matrix is: {:?}", se3.clone().rot_mat());
    println!("The translation vector is: {:?}", se3.t().clone());
    let vb = se3.parameters();
    println!("the vector is {:?}", &vb);
    let se32: SE3<f64> = SE3::<f64>::try_from_parameters(vb.clone()).unwrap();
    println!("The SpecialOrthogonalMatrix3 matrix is: {:?}", se32.clone().rot_mat());
    println!("The translation vector is: {:?}", se32.t().clone());
    for i in 0..6 {
        assert!((vb[i] - vs[i]).abs() < 1e-10);
    }
}

#[test]
fn test_parameters_se3() {
    use optimization::{jacobian, FiniteDifference};
    use algebra_traits::{TrySub, Parameters};
    use algebra::matrix::MatrixDyn;
    let vs = vec![0.0, 0.0, 0.0, 0.0, 0.0, 1.0];

    let f =
        |p: &Vec<f64>| SE3::<f64>::try_from_parameters(p.clone()).unwrap();
    let jac = jacobian(&f, &vs, FiniteDifference::default());
    // println!("jac:{}",jac);

    // let jac_round=jac.map(|z|z.round());
    // println!("jac_round:{}",jac_round);

    assert!((jac.try_sub(MatrixDyn::<f64>::identity(6)).unwrap()).max_norm_of_entries() < 1e-6);
}
