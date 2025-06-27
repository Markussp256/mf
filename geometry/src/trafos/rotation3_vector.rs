
// rotation that is described by an axis through the origin with its length equal to the rotation angle
// note that this way we circumvent the problem that rotation axis and angle can be flipped simultaneously to get the same rotation

use std::ops::{Mul, Neg};

use std::fmt::{Debug, Display, Formatter};


use container_traits::{for_static::FromFn, IntoInner, IntoParameters, for_static::TryFromParameters};
use matrix::Matrix3;
use matrix_traits::{identity::for_static::Identity,MatrixConstructError};
use matrix_wrappers::Orthogonal;
use num_traits::{One, Pow, Zero};

use algebra_traits::*;
use algebra_traits::operators::div_by_small_natural::Div2;

use geometry_traits::transformation::ContradictingDataForApproximatingTrafoError;

use phys_units::{generic::Angle, Radians};
use algebra::quaternion::ProjectiveQuaternion;

use crate::primitives::{UnitVector3, Vector3};
use crate::Vector;

type O3   <F>=matrix_wrappers::OrthogonalMatrix<F,3>;
type SO3  <F>=matrix_wrappers::SpecialOrthogonalMatrix<F,3>;
type Skew3<F>=matrix_wrappers::SkewSymmetricMatrix<F,3>;

#[derive(Clone, Debug)]
pub struct ContradictingDataForRotation3Vector<F:RealNumber, V:Vectorspace<F>> {
    pub so:SO3<F>,
    pub x:Vector3<V>,
    pub y:Vector3<V>,
    pub yalt:Vector3<V>,
    pub d:Nonnegative<F::RealType>,
    pub tol:Nonnegative<F::RealType>
}

impl<F:Debug+RealNumber, V:Debug+Vectorspace<F>> Display for ContradictingDataForRotation3Vector<F,V> where F::RealType:Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Contradicting data to estimate Rotation3Vector transformation:
        estimating SO3-matrix with orthogonal Procrustes is M={:?}. We have 
        {:?}=f({:?}), M*{:?}={:?} but {:?}.distance({:?})={:?}>tol={:?}",
        self.so, self.y, self.x, self.x, self.yalt, self.yalt, self.y, self.d, self.tol)
    }
}

impl<F:RealNumber+Debug, V:Debug+Vectorspace<F>> std::error::Error for ContradictingDataForRotation3Vector<F,V> where F::RealType:Debug {}
impl<F:RealNumber+Debug, V:Debug+Vectorspace<F>> ContradictingDataForApproximatingTrafoError for ContradictingDataForRotation3Vector<F,V> where F::RealType:Debug {}


#[derive(
    Clone,
    PartialEq,
    Debug,
    container_derive::IntoParameters,
    container_derive::TryFromParameters,
)]
pub struct ScaledRotationAxis<F>(Vector3<F>); //  where F:'static

impl<F> ScaledRotationAxis<F> {
    pub fn new(v:Vector3<F>) -> Self {
        Self(v)
    }

    // we can not implement zero because then we would need to implement add as well
    pub fn zero() -> Self where F:Zero {
        Self(Vector3::zero())
    }

    pub fn is_zero(&self) -> bool where F:Zero {
        self.0
            .is_zero()
    }

    pub fn vector(&self) -> &Vector3<F> {
        &self.0
    }

    pub fn into_vector(self) -> Vector3<F> {
        self.0
    }
}

impl<F:Neg<Output=F>> Neg for ScaledRotationAxis<F> {
    type Output=Self;

    fn neg(self) -> Self::Output {
        Self(-self.0)
    }
}

impl<F:Clone+RealNumber> From<ProjectiveQuaternion<F>> for ScaledRotationAxis<F> {
    fn from(pq:ProjectiveQuaternion<F>) -> Self {
        let q=pq.unit_quaternion();
        let qimag=q.imag();
        let axssina2:Vector3::<F>=Vector3::from_fn(|i|qimag[i].clone());
        if axssina2.is_zero() {
            return Self::zero();
        }
        // half of the angle
        let a2=<F as TryATan2>::try_atan2(axssina2.clone().norm().into_signed(),
                                             q.real().clone()).unwrap();
        Self::new(axssina2.try_div(a2.sinc().div2()).unwrap())
    }
}

#[test]
fn test_scaled_rotation_axis_from_projective_quaternion() {
    use algebra_traits::Tolerance;
    let proj_quat=ProjectiveQuaternion::<f64>::try_from_real_imag(0.0, [0.0, 0.0, 1.0]).unwrap();
    let scaled_rotation_axis=ScaledRotationAxis::from(proj_quat);
    assert!((scaled_rotation_axis.vector().z()-std::f64::consts::PI).is_small());
}


// we dont implement from<rotation3vector> for projectivequaternion because we dont want algebra to depend on geometry
impl<F:Clone+RealNumber> Into<ProjectiveQuaternion<F>> for Rotation3Vector<F> where Angle<F> : Radians<F> {
    fn into(self) -> ProjectiveQuaternion<F> {
        match self.clone().try_into_axis_dir() {
            Some(axis) => {
                let sa2=self.abs_angle().div2();
                let imag:[F;3]=(axis * F::from(sa2.clone().sin())).into();
                let real=F::from(sa2.cos());
                ProjectiveQuaternion::try_from_real_imag(real, imag).ok().unwrap()
            },
            None => ProjectiveQuaternion::one()
        }
    }
}

impl<F:Clone+RealNumber> Into<SO3<F>> for Rotation3Vector<F> {
    fn into(self) -> SO3<F> {
        // some constants we will need
        let angle = self.clone().abs_angle();
        let sinc_angle = angle.clone().sinc();
        let sinc_angle2 = angle.clone().div2().sinc();
        let skew2_fac = sinc_angle2.pow2().div2();
        let skew:Matrix3<F>=self.into_skew().into_inner();
        // rodrigues formula but K is scaled with angle
        // second factor is (1-cos(angle))/angle^2=sinc^2(angle/2)/2
        SO3::try_new(
            Orthogonal::try_new(
                Matrix3::<F>::identity()
                    +skew.clone().scalar_mul(&F::from(sinc_angle))
                    +skew.pow2().scalar_mul(&F::from(skew2_fac))).unwrap(),
                F::one()).ok().unwrap()
    }
}

#[test]
fn test_try_into_so3() {
    use utils::IntoThis;
    let rot3vector=Rotation3Vector::new(ScaledRotationAxis::new(Vector3::new(1.0, 2.0, 3.0)));
    let _rot3v=rot3vector.into_this::<SO3::<f64>>();
}

/// Rotation that transforms direction corresponding to a to direction corresponding to b with the smallest possible angle
impl<F:RealNumber> Rotation3Vector<F> {
    pub fn try_rot_that_trafos_a2b(a:Vector3<F>, b:Vector3<F>) -> Option<Self> {
        let a: Vector3<F>=a.try_normalize().ok()?.1;
        let b: Vector3<F>=b.try_normalize().ok()?.1;
        let axb:Vector::<F, 3>=a.clone().cross_product(b.clone());
        //let axb: Vector3<F>=-(a.cross_product(b));
        let cos_angle=a.scalar_product(b);
        let angle=<F as TryATan2>::try_atan2(axb.clone().norm().into_signed(), cos_angle).unwrap();
        Some(Rotation3Vector::new(ScaledRotationAxis::new(axb.try_div(angle.sinc()).unwrap())))
    }
}

#[cfg(test)]
use matrix_traits::MatrixMatrixProduct;

#[test]
fn test_from_scaled_axis() {
    let a=Vector::<f64,3>::new(2.3,-2.0, 0.2);
    let rot1:SO3<f64>=Rotation3Vector::new(ScaledRotationAxis::new(a.clone())).into();
    let rot2:SO3<f64>=Rotation3Vector::new(ScaledRotationAxis::new(a.scalar_mul(&2.0))).into();  
    assert!(<SO3<f64> as matrix_traits::Matrix>::is_close_to(rot2, rot1.clone().matrix_matrix_product(rot1)));
}

#[test]
fn test_from_scaled_axispi() {
    use phys_units::{Revolutions,Radians};
    // create vector of norm 2*pi
    let a=Vector3::new(2.3,-2.0, 0.2);
    let an: Vector3<f64>=a.try_normalize().unwrap().1;
    let a: Vector3<f64>=an.scalar_mul(&phys_units::Angle::from_rev(0.5).rad());
    let rot:SO3<f64>=Rotation3Vector::new(ScaledRotationAxis::new(a.clone())).into();

    // reversed, i.e. scaled axis from matrix
    let aest=Rotation3Vector::from(rot.clone()).scaled_rotation_axis().vector().clone();

    assert!((a.clone()-aest.clone()).norm() < 1e-10 || (a.clone()+aest).norm() < 1e-10);

    let an:Vector3<f64>=a.try_normalize().unwrap().1;
    let m=Matrix3::from_fn(|(i,j)|2.0*an[i]*an[j])-Matrix3::<f64>::identity();

    println!("m: {}", m);
    println!("rot: {}", rot);

    let mnoe=(rot.into_inner()-m).try_max_norm_of_entries().unwrap();
    println!("{}", mnoe);
    assert!(mnoe < 1e-10);
}

#[test]
fn test_from_scaled_axis2pi() {
    use phys_units::{Revolutions,Radians};
    // create vector of norm 2*pi
    let a=Vector3::new(2.3,-2.0,0.2);
    let an:Vector3<f64>=a.try_normalize().unwrap().1;
    let a:Vector3<f64>=an*phys_units::Angle::from_rev(1.0).rad();
    let rot:SO3<f64>=Rotation3Vector::new(ScaledRotationAxis::new(a)).into();

    let res=rot.into_inner()-Matrix3::<f64>::identity();
    println!("res: {}", res);
    let mnoe=res.try_max_norm_of_entries().unwrap();
    println!("mnoe: {}", mnoe);
    assert!(mnoe < 1e-10);
}

#[test]
fn test_rot_that_trafos_a2b() {
    use algebra_traits::Distance;
    // let a=Vector3::new(1.0,0.0,0.0);
    // let b=Vector3::new(1.0,0.1,0.0);
    let a=Vector3::new(2.3,-2.0,0.2);
    let b=Vector3::new(0.1,1.5,0.4);
    let rot=Rotation3Vector::try_rot_that_trafos_a2b(a.clone(), b.clone()).unwrap();
    let rota=rot*a;
    let normalized_uw=|z:Vector3<f64>|z.try_normalize::<Vector3<f64>>().unwrap().1;
    let b:Vector3::<f64>=normalized_uw(b);
    let rota=normalized_uw(rota);
    assert!(b.distance(rota) < 1e-8);
}




#[derive(
    Clone,
    PartialEq,
    Debug,
    derive_more::Into,
    derive_more::From,
    container_derive::IntoParameters,
    container_derive::TryFromParameters
)]
pub struct Rotation3Vector<F>(ScaledRotationAxis<F>);

impl<F> Rotation3Vector<F> {
    pub fn new(scaled_axis:ScaledRotationAxis<F>) -> Self {
        Self(scaled_axis)
    }

    pub fn from_vector(vector:Vector3<F>) -> Self {
        Self(ScaledRotationAxis::new(vector))
    }

    pub fn zero() -> Self where F: Zero {
        Self::from_vector(Vector3::zero())
    }

    pub fn is_zero(&self) -> bool where F : Zero {
        self.0
            .is_zero()
    }

    pub fn scaled_rotation_axis(&self) -> &ScaledRotationAxis<F> {
        &self.0
    }

    pub fn into_scaled_rotation_axis(self) -> ScaledRotationAxis<F> {
        self.0
    }

    pub fn is_identity(&self) -> bool where F:Zero {
        self.0.is_zero()
    }
}

impl<F:Neg<Output = F>> num_traits::Inv for Rotation3Vector<F> {
    type Output=Self;

    fn inv(self) -> Self::Output {
        Self(-self.0)
    }
}


impl<F:Clone+RealNumber> Rotation3Vector<F> {

    pub fn into_skew(self) -> Skew3<F> {
        <Skew3<F> as TryFromParameters<F,MatrixConstructError>>::try_from_iter(
            self.0
                .into_parameters()).unwrap()
    }

    pub fn try_into_axis_dir(self) -> Option<UnitVector3<F>> {
       self.0
           .into_vector()
           .try_normalize().ok()
           .map(|s|s.1)
    }
}

impl<F:Scalar<RealType=R>, R:RealNumber> Rotation3Vector<F> {
    pub fn abs_angle(&self) -> Angle<R>
    where Vector3<F> : Clone+Norm<NormT=R>,
    Angle<R> : Radians<R> {
        Angle::from_rad(self.scaled_rotation_axis()
                                 .vector()
                                 .clone()
                                 .norm()
                                 .into_signed()
                                 .into())
    }
}

impl<R:RealNumber> Rotation3Vector<R> {
    pub fn into_parameters_as_angles(self) -> [Angle<R>;3] where  Angle<R> : Radians<R>{
        <[R;3]>::try_from_iter(self.into_parameters()).unwrap()
            .map(Angle::from_rad)
    }
}

impl<F:Scalar> Pow<F> for Rotation3Vector<F> {
    type Output=Self;
    fn pow(self, f:F) -> Self {
        Self(ScaledRotationAxis(self.0.0.scalar_mul(&f)))
    }
}


impl<F:Clone+Mul<V,Output=V>+Scalar,
     V:'static+Clone+TryDiv<Output=F>+Vectorspace1d> Mul<Vector3<V>> for Rotation3Vector<F> {
    type Output=Vector3<V>;

    fn mul(self, rhs: Vector3<V>) -> Self::Output {
        // rodrigues formula
        let angle:Angle<F::RealType>=self.abs_angle();
        let sa: Vector<F, 3>=
            self.into_scaled_rotation_axis()
                .into_vector();
        let cp=sa.clone().cross_product(rhs.clone());
        let sp=V::any_linear_combination(sa.clone(),rhs.clone()).unwrap();
        let sinc_angle2:F::RealType= angle.clone().div2().sinc();
        let skew2_fac = sp.scalar_mul(&sinc_angle2.pow2().div2().into());
        rhs.scalar_mul(&F::from(angle.clone().cos()))
        +cp.scalar_mul(&F::from(angle.sinc()))
        +sa*skew2_fac // +sa.map(|sai|sai*skew2_fac.clone())
    }
}

#[cfg(test)]
use matrix_traits::{MatrixVectorProduct, TryFromMatrix};

#[test]
fn test_apply_vector() {
    let rot3=Rotation3Vector::try_from_iter([0.0, 0.0, 1e-3]).unwrap();
    let so3:SO3<f64>=rot3.clone().try_into().unwrap();//::try_from_s(rot3);
    let v=Vector3::<f64>::new(1.0, 0.0, 0.0);
    let rot3v=rot3*v.clone();
    let v_alg:algebra::Vector3<f64>=v.clone().into();
    let so3v_alg=so3.matrix_vector_product(v_alg);
    let so3v=Vector3::from(so3v_alg);

    println!("rot3v {}", rot3v);
    println!("so3v {}", so3v);

    assert!((rot3v-so3v).is_small());
}

impl<F:Clone+Scalar> Mul<UnitVector3<F>> for Rotation3Vector<F> {
    type Output=UnitVector3<F>;
    fn mul(self, rhs: UnitVector3<F>) -> Self::Output {
        let rhsv:Vector3<F>=rhs.into();
        (self*rhsv).try_into()
                   .ok().unwrap()
    }
}



// impl<F:Scalar+Mul<V,Output=V>, V:Clone+TryDiv<Output=F>+InnerProductSpace1d> Transformation<F,Vector3<V>,Vector3<V>> for Rotation3Vector<F> {
//     fn apply(&self, x:&Vector3<V>) -> Vector3<V> {
//         self.clone()*x.clone()
//     }

//     fn defining_points() -> Vec<Vector3<V>> {
//         let v_nz=V::NonZero.into_inner();
//         vec![0,1].into_iter()
//                  .map(|i|Vector3::try_put_at(i, v_nz.clone()).unwrap())
//                  .collect()
//     }

//     fn try_approx_with_weights(orig_imag_pairs:Vec<(F, Vector3<V>, Vector3<V>)>) -> Result<Self,ApproximationTrafoError> {
//         // orthogonal procrustes
//         // https://en.wikipedia.org/wiki/Orthogonal_Procrustes_problem
//         let mut atb=Matrix::<F,3,3>::zero();
//         let mut xs=Vec::new();

//         for (w, x, y) in orig_imag_pairs.clone() {
//             xs.push(x.clone());
//             let x=x.div_nz().into_this::<algebra::Vector<F,3>>().scalar_mul(&w);
//             let y=y.div_nz().into_this::<algebra::Vector<V,3>>().scalar_mul(&w);
//             atb=atb+Matrix::<f64,3,1>::from(y).conjugate()*Matrix::<f64,3,1>::from(x).transpose()
//         }
//         if SubSpace::new(xs).0.dimension() < 2 {
//             return Err(ApproximationTrafoError::InsufficientData);
//         }
//         let so3=atb.project()
//                    .into_this::<OrthogonalMatrix<f64, 3>>()
//                    .try_into_this::<SO3<f64>>()
//                    .unwrap_or_else(|_|panic!(""));
//         // let tol=tol.unwrap_or(1e-10);
//         // // check
//         // for (_,x,y) in orig_imag_pairs.clone() {
//         //     let yalt=so3.clone() * x.clone();
//         //     let d=yalt.distance(&y);
//         //     if d > tol {
//         //         let cd4r3v= ContradictingDataForRotation3Vector{
//         //             so: so3.clone(), x, y, yalt, d, tol};
//         //         return Err(ApproximationTrafoError::ContradictingData(Box::new(cd4r3v)));
//         //     }
//         // }
//         Ok(so3.into())
//     }
    
//     fn try_approx(orig_imag_pairs:Vec<(Vector3<V>,Vector3<V>)>) -> Result<Self,ApproximationTrafoError> {
//         Self::try_approx_with_weights(orig_imag_pairs.into_iter().map(|xy|(<F>::one(), xy.0, xy.1)).collect())
//     }
    
//     fn try_new(f: impl Fn(&Vector3<V>) -> Vector3<V>) -> Result<Self,ApproximationTrafoError> {
//         let pts_iter=||Self::defining_points().into_iter();
//         Self::try_approx_with_weights(pts_iter().zip(pts_iter()).map(|(pt0,pt1)|(<F>::one(), pt0, f(&pt1))).collect())
//         .map_err(|e|match e {
//             // if there is insufficient data its because defining points does not contain enough points
//             ApproximationTrafoError::InsufficientData => ApproximationTrafoError::DefiningPointsNotCorrect,
//             err => err
//             })
//     }
    
//     fn try_composition<Mid,
//                        TLhs:Transformation<F,Mid,Vector3<V>>,
//                        TRhs:Transformation<F,Vector3<V>,Mid>>(lhs:TLhs, rhs:TRhs) -> Result<Self, ApproximationTrafoError> {
//         Self::try_new(|x:&Vector3<V>|lhs.apply(&rhs.apply(x)))
//     }
    
//     fn images(&self) -> Vec<Vector3<V>> {
//         Self::defining_points()
//                 .iter()
//                 .map(|pt|self.apply(pt))
//                 .collect()
//     }
    
//     fn try_inverse<T:Transformation<F,Vector3<V>,Vector3<V>>>(&self) -> Result<T, ApproximationTrafoError> {
//         T::try_approx_with_weights(
//             Self::defining_points()
//                                 .into_iter()
//                                 .zip(self.images())
//                                 .map(|(pt,pt_img)|(<F>::one(), pt_img, pt))
//                                 .collect())
//     }
    
//     fn try_from(other:impl Transformation<F,Vector3<V>,Vector3<V>>) -> Result<Self,ApproximationTrafoError> {
//         Self::try_new(|x:&Vector3<V>|other.apply(x))
//     }
// }


utils::from_via!(impl<R:RealNumber> From<ProjectiveQuaternion<R>> for Rotation3Vector<R>, via ScaledRotationAxis<R>);

utils::from_via!(impl<R:RealNumber> From<SO3<R>> for Rotation3Vector<R>, via ProjectiveQuaternion<R>);

#[test]
fn test_rot3vector_from_scaled_rotation_axis() {
    let scaled_rotation_axis=ScaledRotationAxis::new(Vector3::new(0.0, 0.0, std::f64::consts::PI));
    let rot3vector=Rotation3Vector::from(scaled_rotation_axis);
    assert!((rot3vector.abs_angle().rad() - std::f64::consts::PI).is_small());
}

#[test]
fn test_rot3vector_from_projective_quaternion() {
    let proj_quat=ProjectiveQuaternion::<f64>::try_from_real_imag(0.0, [0.0, 0.0, 1.0]).unwrap();
    let rot3vector=Rotation3Vector::from(proj_quat);
    assert!((rot3vector.abs_angle().rad() - std::f64::consts::PI).is_small());
}


#[test]
fn test_rot3vector_from_so3() {
    
    let so3=SO3::<f64>::try_new(O3::try_from_matrix( matrix::matrix![-1.0, 0.0, 0.0;0.0, -1.0, 0.0; 0.0, 0.0, 1.0]).unwrap(),1.0).unwrap();
    let rot3vector=Rotation3Vector::from(so3);
    assert!((rot3vector.abs_angle().rad() - std::f64::consts::PI).is_small());
}


// impl<R:RealNumber> From<SO3<R>> for Rotation3Vector<R> {
//     fn from(so3:SO3<R>) -> Self {
//             let pq:ProjectiveQuaternion<R>=so3.into();
//             pq.into()
//             // function that mapes scaled axis to residual
//             // let f=|x:&Rotation3Vector<R>|{
//             //     x.clone()
//             //      .try_into_this::<SO3<R>>()
//             //      .ok().unwrap()
//             //      .into_matrix()-m.clone()
//             // };
//             // optimization::fsolve(&f, &rough_scaled_axis, None)
//             //     .ok()
//             //     .unwrap()
//     }
// }

#[test]
fn test_parameters_rot3() {
    use optimization::{jacobian, FiniteDifference};
    // use container_traits::for_static::TryParameters;
    use matrix::MatrixDyn;
    // use crate::optimization::{FiniteDifference, FiniteDifferenceMethod};

    let vs = [0.0, 0.0, 0.0];
    let f = |p: [f64;3]| {
        <SO3<f64> as TryFromParameters<f64,MatrixConstructError>>::try_from_iter(p).unwrap()
    };

    // let f0=f(&vs);
    // // println!("f0: {:?}",f0.parameters());
    // println!("fepsx: {:?}",f(&[1e-4,0.0,0.0]).parameters());
    // println!("fepsy: {:?}",f(&[0.0,1e-4,0.0]).parameters());
    // println!("fepsz: {:?}",f(&[0.0,0.0,1e-4]).parameters());

    let jac = jacobian(&f, vs, FiniteDifference::<f64>::default());
    println!("{}", jac);
    let id=Matrix3::<f64>::identity();
    assert!(<MatrixDyn<f64> as matrix_traits::Matrix>::is_close_to(jac, id));
}