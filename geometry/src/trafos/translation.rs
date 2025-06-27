
use algebra_traits::*;
use container_traits::IntoSum;
use geometry_traits::transformation::{
    Transformation, ApproximationTrafoError, ContradictingDataForApproximatingTrafoError
};

use std::ops::{Add, Mul, Sub};
use num_traits::Zero;

use crate::Vector;

use std::fmt::{Debug, Display, Formatter};

#[derive(Clone, Copy, Debug)]
pub struct ContradictingDataForTranslation<R, A> {
    pub x0:A,
    pub y0:A,
    pub x1:A,
    pub y1:A,
    pub d:R,
    pub tol:R
}

impl<R:Debug, A:Debug> Display for ContradictingDataForTranslation<R, A> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Contradicting data to estimate translation: ({:?}-{:?}).distance({:?}-{:?})={:?}>tol={:?}",self.y0,self.x0,self.y1,self.x1,self.d,self.tol)
    }
}

impl<R:Debug, A:Debug> std::error::Error for ContradictingDataForTranslation<R, A> {}
impl<R:Debug, A:Debug> ContradictingDataForApproximatingTrafoError for ContradictingDataForTranslation<R, A> {}


// due to Translation owning an element of type A::V and not A some of the methods need to be implemented separately

#[derive(Clone, 
         Copy, 
         Debug,
         PartialEq,
         derive_more::From,
         algebra_derive::AdditiveGroup,
         container_derive::NumberOfDegreesOfFreedom,
         container_derive::IntoParameters,
         container_derive::TryFromParameters,
         serde::Serialize,
         serde::Deserialize)]
pub struct Translation<V>(V);


impl<V:Display> Display for Translation<V> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "Translationvector:")?;
        self.0.fmt(f)
    }
}

// impl<V> AnyAdd for Translation<V> where Self : std::ops::Add<Output=Self> {
//     type Output=Self;
//     fn any_add(self, rhs:Self) -> Result<Self::Output,AddError> {
//         Ok(self + rhs)
//     }
// }

impl<V:ScalarMul<F>, F> Mul<F> for Translation<V> {
    type Output=Self;
    fn mul(self, rhs: F) -> Self::Output {
        Translation(self.0.scalar_mul(&rhs))
    }
}

// impl<A:Affinespace> Debug for Translation<A> where A::V:Debug {
//     fn fmt(&self, f: &mut Formatter<'_>) -> Result {
//         f.debug_tuple("Translation").field(&self.0).finish()
//     }
// }

// impl<A:Affinespace> Clone for Translation<A> where A::V:Clone {
//     fn clone(&self) -> Self {
//         Self::new(self.0.clone())
//     }
// }

// impl<A:Affinespace+Nonempty+PartialEq> PartialEq for Translation<A> {
//     fn eq(&self, other: &Self) -> bool {
//         A::an_element()+self.0 == A::an_element()+other.0
//     }
// }

// impl<A:Affinespace+Nonempty+Parameters> Parameters for Translation<A> {
//     fn parameters(&self) -> Vec<f64> {
//         (A::an_element()+self.0).parameters()
//     }

//     fn try_from_iter<I: Iterator<Item = f64>>(iter: &mut I) -> Option<Self> {
//         A::try_from_iter(iter).map(|v|Self::new(v-A::an_element()))
//     }
// }


impl<V> Translation<V> {
    pub fn new(v:V) -> Self {
        Self(v)
    }

    pub fn from_to<A:Sub<Output=V>>(from:A, to:A) -> Self {
        Self(to-from)
    }

    pub fn vector(&self) -> &V {
        &self.0
    }

    pub fn into_vector(self) -> V {
        self.0
    }

    fn apply<A:Add<V,Output=A>>(&self, x:A) -> A where V : Clone {
        x+self.0.clone()
    }
}

// impl<A:Affinespace+Nonempty> Clone for Translation<A> {
//     fn clone(&self) -> Self {
//         let se=A::an_element();
//         let set=self.apply(se);
//         Self(set-se)
//     }
// }

utils::from_via!(impl<T, const N:usize> From<algebra::Vector<T,N>> for Translation<Vector<T,N>>, via Vector<T,N>);

#[cfg(feature = "nalgebra_support")]
impl<V, const N:usize> From<nalgebra::Translation<V,N>> for Translation<Vector<V,N>> where V:nalgebra::Scalar {
    fn from(value: nalgebra::Translation<V,N>) -> Self {
        Self::new(value.vector.into())
    }
}

#[cfg(feature = "nalgebra_support")]
impl<V, const N:usize> Into<nalgebra::Translation<V,N>> for Translation<Vector<V,N>> where V:nalgebra::Scalar {
    fn into(self) -> nalgebra::Translation<V,N> {
        nalgebra::Translation::<V,N>{vector:self.0.into()}
    }
}

impl<R: Clone+RealNumber,
     A: Clone+Sub<Output=V>+MetricTorsor<DistT=DistT>+'static+ConstElement,
     V: Clone+Vectorspace<R>+Norm,
     DistT: Zero+Max+PartialOrd> Transformation<R, A, A> for Translation<V> {

    fn apply(&self, x:A) -> A {
        self.apply(x)
    }

    fn try_approx_with_weights(orig_imag_pairs:Vec<(R, A, A)>) -> Result<Self,ApproximationTrafoError> {
        let ws:Vec<R>=orig_imag_pairs.iter().map(|(w,_,_)|w.clone()).collect();
        let sum:R=ws.clone().into_sum();
        if sum <= R::zero() {
            return Err(ApproximationTrafoError::SumOfWeightsNonpositive);
        }
        let ds:Vec<V>=orig_imag_pairs.clone().into_iter().map(|(_,x,y)|y-x).collect();
        // let tol=<Self as Transformation<A,A>>::tol_y();
        // for i in 0..ds.len()-1 {
        //     for j in i+1..ds.len() {
        //         let d=ds[i].clone().distance(ds[j].clone());
        //         if  d > tol {
        //             let cd4trans=ContradictingDataForTranslation{
        //                 x0:orig_imag_pairs[i].clone().1,
        //                 y0:orig_imag_pairs[i].clone().2,
        //                 x1:orig_imag_pairs[j].clone().1,
        //                 y1:orig_imag_pairs[j].clone().2,
        //                 d:d.into(),
        //                 tol:tol.clone()};
        //             return Err(ApproximationTrafoError::ContradictingData(Box::new(cd4trans)));
        //         }
        //     }
        // }
        Ok(Self::new(V::linear_combination(ws.into_iter().zip(ds.into_iter())).try_scalar_div(&sum).unwrap()))
        //Ok(Self::new(sum(ws.iter().zip(ds.iter()).map(|(w,d)|d.clone()*w.clone()))/sum))
    }
    
    fn defining_points() -> impl ExactSizeIterator<Item=A> {
        std::iter::once(A::ELEMENT)
    }
    
    fn try_approx(orig_imag_pairs:Vec<(A,A)>) -> Result<Self,ApproximationTrafoError> {
        Self::try_approx_with_weights(orig_imag_pairs.into_iter().map(|xy|(<R>::one(), xy.0, xy.1)).collect())
    }
    
    fn try_new(f: impl Fn(A) -> A) -> Result<Self,ApproximationTrafoError> {
        let pts_iter=||Self::defining_points().into_iter();
        Self::try_approx(pts_iter().zip(pts_iter()).map(|(pt0,pt1)|(pt0, f(pt1))).collect())
        .map_err(|e|match e {
            // if there is insufficient data its because defining points does not contain enough points
            ApproximationTrafoError::InsufficientData => ApproximationTrafoError::DefiningPointsNotCorrect,
            err => err
            })
    }
    
    fn try_composition<Mid,
                       TLhs:Transformation<R,Mid,A>,
                       TRhs:Transformation<R,A,Mid>>(lhs:TLhs, rhs:TRhs) -> Result<Self, ApproximationTrafoError> {
        Self::try_new(|x:A|lhs.apply(rhs.apply(x)))
    }
    
    fn images(&self) -> impl ExactSizeIterator<Item=A> {
        Self::defining_points()
                .map(|pt|self.apply(pt))
    }
    
    fn try_inverse<T:Transformation<R,A,A>>(&self) -> Result<T, ApproximationTrafoError> {
        T::try_approx_with_weights(
            Self::defining_points()
                                .into_iter()
                                .zip(self.images())
                                .map(|(pt,pt_img)|(<R>::one(), pt_img, pt))
                                .collect())
    }
    
    fn try_from(other:impl Transformation<R,A,A>) -> Result<Self,ApproximationTrafoError> {
        Self::try_new(|x:A|other.apply(x))
    }
}

#[test]
fn test_parametrize() {
    use container_traits::for_static::TryFromParameters;
    use crate::Vector3;
    let vec=vec![1.0,3.4,2.3];
    let t=Translation::<Vector3<f64>>::try_from_iter(vec.clone()).unwrap();
    assert_eq!(t.vector(),&Vector3::<f64>::try_from_iter(vec).unwrap());
}



// use crate::algebra::{Abs,Vectorspace};
// use crate::geometrie::{Arrow,Vector};

// pub trait Translation<Point:Abs<Diff=V>+PartialEq, A:Arrow<Point>,V:Vector<Point,A,F>, F > : Vectorspace<F> {
//     // required methods
//     fn new(v:V) -> Self;
//     fn vector(&self) -> V;

//     // provided methods
//     fn apply(&self, pt:Point) -> Point {
//         pt+self.vector()
//     }

// }

// pub trait Translation : From<Self::V>
//                        +Into<Self::V> {
//     type V;
//     type P;
// }

// impl<T:Translation> Transformation for T where T::P: Nonempty {
//     type X=T::P;
//     type Y=T::P;

//     fn apply(&self, x:T::P) -> T::P {
//         x+self.into()
//     }

//     fn try_approx_with_weights(orig_imag_pairs:Vec<(f64,T::P,T::P)>, tol:Option<f64>) -> Result<Self,super::ApproximationTrafoError> {
//         let weights=orig_imag_pairs.iter().map(|(&w,_,_)|w);
//         let sum_weights:f64=weights.sum();
//         if sum_weights == 0.0 {
//             return Err(super::ApproximationTrafoError::SumOfWeightsNonpositive);
//         }
//         let vs=orig_imag_pairs.into_iter().map(|(_,x,y)|y-x);
//         let vs_avg=T::V::linear_combination(weights.zip(vs).collect());
//         // let vs_avg:V=weights.iter().zip(vs.iter()).fold(V::zero(),|acc,(&w,&v)|acc+w*v)/sum_weights;
//         match tol {
//             Some(tol) => {
//                 for vi in vs {
//                     if vi.dist(vs_avg) > tol {
//                         return Err(super::ApproximationTrafoError::CouldNotFindTrafoForTol(tol));
//                     }
//                 }
//             },
//             None => {}
//         }
//         Ok(vs_avg.into())
//     }

//     fn defining_points() -> Vec<T::P> {
//        vec![T::P::an_element()]
//     }
    

// }