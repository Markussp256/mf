
use std::ops::{Mul, Sub};

use algebra_traits::{TrySub, ConstElement, ConstNonZero, Max, Norm, Origin, RealNumber, ScalarMul, Torsor, TryDiv, TryNormalize, Vectorspace1d};
use algebra::Unit;

use container_traits::{for_static::X, FromContainer, Map, Parameter};

use matrix_traits::{identity::for_static::Identity, IntoMatrix, MatrixDynamic, MatrixVectorProduct, Transpose, TryIntoMatrix};

use matrix::{row_col::{MatrixCol2, MatrixCol3}, Matrix, Matrix2, MatrixDyn};

use matrix_decompositions::qr::{OrthogonalQR, HouseholderTrafo};

use matrix_wrappers::SpecialOrthogonalMatrix;

use utils::IntoThis;
use vector_and_affine_spaces::SubSpaceDyn;
use crate::{Point3, Vector3};
use super::{Rotation3Point, Rotation3Vector, Translation, SE3};

use num_traits::{Pow, Zero};
use std::fmt::Debug;

type SO2<F>=SpecialOrthogonalMatrix<F,2>;

pub struct ScrewMotion<F, A:Sub> {
    rotation:Rotation3Point<F, A>,
    parallel_translation:Translation<Vector3<<A as Sub>::Output>>
}

impl<F:Clone, A:Clone+Sub<Output=V>, V:Clone> Clone for ScrewMotion<F, A> {
    fn clone(&self) -> Self {
        Self{rotation:self.rotation.clone(),
             parallel_translation:self.parallel_translation.clone()}
    }
}

impl<F:Debug, A:Debug+Sub<Output=V>,V:Debug> Debug for ScrewMotion<F, A> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ScrewMotion")
         .field("rotation", &self.rotation)
         .field("parallel_translation", &self.parallel_translation)
         .finish()
    }
}

impl<F:Clone+RealNumber,
     A:Sub<Output=V>,
     V:Clone+TryDiv<Output=F>+ConstNonZero> ScrewMotion<F, A> {
    pub fn try_new(rotation:Rotation3Point<F, A>, parallel_translation:Translation<Vector3<V>>) -> Option<Self> {
        // tranlation must be along rotation axis
        let (space,_)=SubSpaceDyn::new(
            [rotation.rot3vector()
                     .scaled_rotation_axis()
                     .vector()
                     .clone(),
            parallel_translation.vector().clone().map(|v|v.div_nz())].into_iter());
        match space.dimension() {
            2 =>  None,
            _ => Some(Self{rotation, parallel_translation}) // 0 | 1
        }
    }

    pub fn abs_angle(&self) -> phys_units::generic::Angle<F> {
        self.rotation
            .abs_angle()
    }
}

impl<F, V:Zero, A:Sub<Output=V>> ScrewMotion<F, A> {
    pub fn rotation_wrt_point(rot: Rotation3Vector<F>, pt: Point3<A>) -> Self {
        Self{rotation:Rotation3Point::new(pt, rot),
             parallel_translation:Translation::zero()}
    }

    pub fn rotation(&self) -> &Rotation3Point<F,A> {
        &self.rotation
    }

    pub fn parallel_translation(&self) -> &Translation<Vector3<V>> {
        &self.parallel_translation
    }

    pub fn into_parts(self) -> (Rotation3Point<F, A>, Translation<Vector3<V>>) {
        (self.rotation, self.parallel_translation)
    }

    pub fn identity() -> Self where A:ConstElement, F:Zero {
        Self{rotation:Rotation3Point::identity(),
             parallel_translation:Translation::zero()}
    }
}

impl<F:RealNumber, V:ScalarMul<F>+TryDiv<Output=F>, A:Sub<Output=V>> Pow<F> for ScrewMotion<F, A> {
    type Output=Self;
    fn pow(self, f:F) -> Self {
        Self{rotation:self.rotation.pow(f.clone()),
             parallel_translation:self.parallel_translation*f.clone()}
    }
}


// <A:Torsor<V=V>+Clone+Parameters1,
//      V:Vectorspace<f64>+Norm<N2=f64>+ConstNonZero+Zero+One+Clone+Div<Output=f64>+Parameters1>

// impl<A:Clone+Sub<Output=V>+Affinespace<F>+Parameter<F>,
//      V:Clone+TryDiv<Output=F>+Vectorspace1d,
//      F:Clone+Mul<V,Output=V>+RealNumber> ScrewMotion<F, A> {
//         pub fn test_from(value: SE3<F,V>) -> (Matrix<F,2,3>, Vector3<F>, Vector3<F>, MatrixDyn<F>)
//         where
//         Point3::<A>:Origin, Vector3<V> : TryNormalize<DivT=<Vector3<V> as Norm>::NormT, DivOutput = Vector3<V>> {
//             let orig=||Point3::<A>::origin();





//             let dir_vec=dir_vec.into_vector();
//             let z=Point3::<A>::origin();
//             let lin_comb_cond=|v:&Point3<A>|dir_vec.clone().scalar_product((v.clone()-z.clone()).div_nz() .map(|v|v.div_nz())); 
//             let f=|p:&Point3<A>|{
//                 let rhs:algebra::Vector3<F>=(value.clone()*p.clone()-p.clone()).map(|v|v.div_nz()).into();
//                 let [mrhs0, mrhs1]=(orth_compl.clone()*rhs).into();
//                 Vector3::new(mrhs0, mrhs1, lin_comb_cond(&p))
//                 // mrhs.try_concat::<1,3>(lin_comb_cond(&p)).unwrap()
//             };
//             let jac=optimization::jacobian::jacobian(&f, &z, FiniteDifference::default());
//             (orth_compl.clone(), dir_vec.clone(), f(&z), jac)
//         }
// }



impl<A : Clone+Sub<Output=V>+Torsor+Parameter<F>+ConstElement,
     V : Clone+TryDiv<Output=F>+TrySub<Output=V>+Vectorspace1d,
     F : 'static+Clone+Mul<V,Output=V>+RealNumber,
     NT: Zero+Clone+Max> From<SE3<F, V>> for ScrewMotion<F, A>
    where Point3<A>  : Origin,
          Vector3<V> : Norm<NormT=NT>+TryNormalize,
          Vector3<F> : 'static {
    fn from(value: SE3<F,V>) -> Self {
        let orig=||Point3::<A>::origin();
        let res=
            value.rot().clone()
                 .into_this::<Rotation3Vector::<F>>()
                 .try_into_axis_dir();
        let dir:Unit<crate::vector::Vector<F, 3>>=match res {
            Some(dir) => dir,
            None => {
                return Self{rotation:Rotation3Point::<F, A>::identity(),
                            parallel_translation:Translation::from(value.clone()*orig()-orig()) }}
        };
        // we consider the restriction of the SE3 to orthogonal complement of dir_vec:
        // if q is 3x2 matrix of orthogonal complement
        // and x |-> ax+b describes SE3
        // then x |-> q^Taqx+q^Tb describes restriction
        // we solve x=q^Taqx+q^Tb => x=(I-q^Taq)^(-1)q^Tb 
        // to find the fixed point
        let hhtd=HouseholderTrafo::try_froma2b(
            Unit::<MatrixCol3::<F>>::from_unchecked(dir),
            Unit::<MatrixCol3::<F>>::ex()).unwrap();
        let mut q:MatrixDyn<F>=hhtd.into_matrix();
        q.try_remove_col(0);
        let q:Matrix<F,3,2>=q.try_into_matrix().ok().unwrap();
        let (a,b)=value.clone().into_matrix_algebra_vector();
        let rot3vector=a.clone().into();
        let qt=q.clone().transpose();
        // since we search for fixpoints we subtract identity
        let qtaq=qt.clone()*a.into_matrix()*q.clone();
        let qtaqmid:Matrix2<F>=qtaq-Matrix2::<F>::identity();
        let qtb:MatrixCol2<V>=qt.matrix_vector_product(b.into());
        let mqtb:MatrixCol2<V>=-qtb;
        let sol:MatrixCol2<V>=qtaqmid.try_solve_least_squares::<MatrixCol2<V>, MatrixCol2<V>, MatrixCol2<V>>(mqtb).unwrap();
        let sol=q*sol;
        let point_on_axis:Point3::<A>=orig()+Vector3::from_container(sol);
        Self{rotation:Rotation3Point::<F, A>::new(point_on_axis.clone(), rot3vector),
             parallel_translation:Translation::from(value*point_on_axis.clone()-point_on_axis.clone())}
    }
}

// #[test]
// fn test_test_from_se3() {
//     use algebra::matrix::SpecialOrthogonalMatrix;
//     // let v=crate::Vector3::<f64>::new(1.2, 3.2, 2.4);
//     // let v=crate::Vector3::<f64>::new(1.0, 2.0, 3.0);
//     let v=crate::Vector3::<f64>::new(0.0, 0.0, 0.0);
//     let so3=
//     SpecialOrthogonalMatrix::try_from(algebra::matrix![1.0, 0.0, 0.0;0.0, 1.0, 0.0; 0.0, 0.0, 1.0]).unwrap();
//     //SpecialOrthogonalMatrix::try_from(algebra::matrix![1.0, 0.0, 0.0;0.0, 0.8, 0.6; 0.0, -0.6, 0.8]).unwrap();
//     let se3=SE3::<f64,f64>::from_parts(so3, v.into());
//     let (a,b,c, jac)=ScrewMotion::<f64,f64>::test_from(se3);
//     println!("orth_compl: {}", a);
//     println!("dir_vec: {}", b);
//     println!("f(0): {}", c);
//     println!("jac: {}", jac);
// }

#[test]
fn test_from_se3() {
    use matrix_wrappers::{OrthogonalMatrix, SpecialOrthogonalMatrix};
    use matrix_traits::TryFromMatrix;
    use phys_units::Radians;
    use algebra_traits::Tolerance;
    // let v=crate::Vector3::<f64>::new(1.2, 3.2, 2.4);
    // let v=crate::Vector3::<f64>::new(1.0, 2.0, 3.0);
    // let v=crate::Vector3::<f64>::new(0.0, 0.0, 0.0);
    let v=crate::Vector3::<f64>::new(2.0, 0.0, 0.0);
    let so3=
    // SpecialOrthogonalMatrix::try_from(algebra::matrix![1.0, 0.0, 0.0;0.0, 1.0, 0.0; 0.0, 0.0, 1.0]).unwrap();
    SpecialOrthogonalMatrix::try_new(
        OrthogonalMatrix::try_from_matrix(
            matrix::matrix!
            [-1.0,  0.0,  0.0;
              0.0, -1.0,  0.0;
              0.0,  0.0,  1.0]).unwrap(),1.0).unwrap();
    // SpecialOrthogonalMatrix::try_from(algebra::matrix![1.0, 0.0, 0.0;0.0, 0.8, 0.6; 0.0, -0.6, 0.8]).unwrap();
    let se3=SE3::<f64,f64>::from_parts(so3, v.into());

    let screw_m: ScrewMotion<f64, f64>=se3.into();

    assert!((screw_m.rotation().abs_angle().clone().rad() - std::f64::consts::PI).is_small());
    assert!((screw_m.rotation().get_point_on_axis().clone() - Point3::<f64>::new(1.0,0.0,0.0)).norm().is_small());
    assert!((screw_m.parallel_translation().into_vector().norm().is_small()))
}




        // match value.try_rot_axis() {
        //     Some(axis) => {
        //         let trans_along_axis=value*axis.point()-axis.point();
        //         let dir=axis.direction();
        //         assert_eq!(1, SubSpace::new(vec![dir, trans_along_axis],1e-10).dimension());
        //         let trans=dir.normalize().scalar_product(trans_along_axis);
        //         return Self{axis:Some(axis), angle:value.abs_angle(), trans}},
        //     None => {
        //         let trans_vec=value*A::an_element()-A::an_element();
        //         match Ray::try_new(orig, trans_vec) {
        //             None => {return Self::identity(); },
        //             Some(ray) => Self{axis:Some}
        //         }

        //         let axis=Ray::new(orig,value.get_t());
        //         let trans=value.get_t().norm();
        //         return Self{axis, angle:phys_units::Angle::zero(), trans}}
        // }