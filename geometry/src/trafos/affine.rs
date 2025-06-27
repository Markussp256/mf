use algebra_traits::*;
use container_traits::{Iter,Parameter};
use geometry_traits::transformation::*;

use vector_and_affine_spaces::AffineBasis;
use crate::{Point, Point3, Vector};

use super::{Translation, SE3};

use std::{fmt::{Debug, Formatter}, ops::{Mul, Sub}};


use num_traits::{One, Zero};

#[derive(Clone, Debug)]
pub struct ContradictingDataForAffineTransformation<F:Scalar, const DIMXV:usize, X, Y>{
    pub ws:Vec<F>,
    pub basis:AffineBasis<F,X, DIMXV>,
    pub images: Vec<Y>,
    pub x:X,
    pub y:Y,
    pub yalt:Y,
    pub d:  Nonnegative<F::RealType>,
    pub tol:Nonnegative<F::RealType>
}

impl<F:Scalar+Debug,
    const DIMXV:usize,
    X:Clone+Sub<Output=XV>+Torsor+Tolerance+Debug,
    XV:Clone+FiniteDimensionalInnerProductSpace<F, DIMXV>,
    Y:Sub<Output=YV>+Torsor+Debug,
    YV:Vectorspace<F>> ContradictingDataForAffineTransformation<F,DIMXV, X,Y> 
    where X::DistT : PartialOrd {

    pub fn lin_comb(&self, f:impl Fn(usize) -> String) -> String {
        let coeffs=self.ws.clone();
        let f=|i:usize|format!("{:?}*{:?}",coeffs[i],f(i));
        let mut string=f(0);
        for i in 1..coeffs.len() {
            string+=&format!("+{:?}",f(i));
        }
        string
    }

    pub fn lin_combs(&self) -> String
        where X           : ConstElement,
              XV::ScProdT : Parameter<F> {
        let basis:&[X;DIMXV]=self.basis
                              .basis();
        let x=|i:usize| format!("{:?}",basis[i]);
        let fx=|i:usize| format!("f({:?})",basis[i]);
        let y=|i:usize| format!("{:?}",self.images[i]);
        format!("f({:?})={:?}={:?}",self.lin_comb(x), self.lin_comb(fx), self.lin_comb(y))
    }
}

impl<F:Scalar+Debug,
     const DIMXV:usize,
     X:Clone+Sub<Output=XV>+Torsor+ConstElement+Debug+Tolerance,
     XV:Clone+FiniteDimensionalInnerProductSpace<F, DIMXV>,
     Y:Sub<Output=YV>+Torsor+Debug,
     YV:Vectorspace<F>> std::fmt::Display for ContradictingDataForAffineTransformation<F, DIMXV, X, Y>
where F::RealType:Debug, 
      X::DistT : PartialOrd,
      XV::ScProdT : Parameter<F> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f,"Contradicting data to estimate affine transformation: we have 
        {:?}=f({:?})={:?}={:?} but {:?}.distance({:?})={:?}>tol={:?}",
        self.y, self.x, self.lin_combs(), self.yalt, self.y, self.yalt, self.d, self.tol)
    }
}

impl<F:Scalar+Debug,
     const DIMXV:usize,
     X:Clone+Sub<Output=XV>+Torsor+ConstElement+Debug+Tolerance,
     XV:Clone+FiniteDimensionalInnerProductSpace<F,DIMXV>,
     Y: Sub<Output=YV>+Torsor+Debug,
     YV:Vectorspace<F>> std::error::Error
     for ContradictingDataForAffineTransformation<F, DIMXV, X, Y> 
     where F::RealType: Debug,
           X::DistT : PartialOrd,
           XV::ScProdT : Parameter<F> {}


impl<F:Scalar+Debug,
const DIMXV:usize,
X:Clone+Sub<Output=XV>+Torsor+ConstElement+Debug+Tolerance,
XV:Clone+FiniteDimensionalInnerProductSpace<F,DIMXV>,
Y: Sub<Output=YV>+Torsor+Debug,
YV:Vectorspace<F>> ContradictingDataForApproximatingTrafoError
     for ContradictingDataForAffineTransformation<F, DIMXV, X, Y>
     where F::RealType: Debug,
    X::DistT : PartialOrd,
    XV::ScProdT : Parameter<F> {}


#[derive(Clone, Debug)]
pub struct AffineTransformation<F, const DIMXV:usize, X, Y=X> {
    affine_basis:AffineBasis<F, X, DIMXV>,
    images:Vec<Y>
}

impl<F,const DIMXV:usize, X, Y> AffineTransformation<F,DIMXV, X,Y> {
    pub fn into_parts(self) -> (AffineBasis<F,X,DIMXV>, Vec<Y>) {
        (self.affine_basis, self.images)
    }
}

impl<F   : Scalar,
     V   : Clone+TryDiv<Output = F>+InnerProductSpace1d<NormT=NT, Norm2T=NT2, ScProdT = SP>,
     SP  : Clone+Zero+Parameter<F>,
     NT  : Zero+Max+Pow2<Output = NT2>+ScalarMul<F::RealType>,
     NT2 : Zero+Max+TrySqrt<Output=Nonnegative<NT>>,
     A   : Zero+One+Clone+Sub<Output=V>+MetricTorsor<DistT=NT>+ConstElement+Tolerance> AffineTransformation<F, 3, Point<A,3>> {
    pub fn from_point_vectors(p:Point<A,3>, vs:[Vector<V,3>;3]) -> Self {
        let images=std::iter::once(p.clone()).chain(vs.into_iter().map(|v|p.clone()+v)).collect();
        Self{affine_basis:AffineBasis::try_new(<Point::<A,3> as algebra_traits::AffineBasis<F>>::affine_basis()).unwrap().0,
             images}
    }
}


impl<F:Scalar,
    const DIMXV:usize, 
     X  : Clone+Sub<Output=XV>+Torsor+ConstElement+Clone+Tolerance,
     XV : Clone+FiniteDimensionalInnerProductSpace<F, DIMXV>,
     Y  : Sub<Output=YV>+Torsor+ConstElement+Clone,
     YV : Vectorspace<F>> AffineTransformation<F, DIMXV, X, Y>
    where X::DistT : PartialOrd,
          XV::ScProdT : Clone+Zero+Parameter<F> {
    fn apply_private(&self, x:X) -> Y {
        let (affine_basis,images)=self.clone().into_parts();
        let coeff=affine_basis.find_coordinates(x);
        Y::try_affine_combination(coeff, images).unwrap()
    }

    pub fn apply_on_vector(&self, v:XV) -> YV {
        self.apply_private(X::ELEMENT+v)
       -self.apply_private(X::ELEMENT)
    }
}


// impl<F:Scalar+Debug,
//      const DIMXV:usize,
//      X:Clone+Sub<Output=XV>+FiniteDimensionalAffineInnerProductSpace<F,DIMXV, DistT=DistTX>+Nonempty+Clone+Debug+'static+Tolerance,
//      XV:Clone+FiniteDimensionalInnerProductSpace<F, DIMXV,NormT=DistTX, ScProdT=ScProdTX>,
//      ScProdTX:Zero+Parameter<F>,
//      DistTX:Pow2+Tolerance,
//      Y:Sub<Output=YV>+ AffineInnerProductSpace<F, DistT=DistTY>+Clone+Debug+'static,
//      YV:Clone+InnerProductSpace<F,NormT=DistTY, ScProdT=ScProdTY>,
//      ScProdTY:Zero+Parameter<F>,
//      DistTY:Pow2+Tolerance> Transformation<F::RealType, X, Y>
//      for AffineTransformation<F, DIMXV, X,Y>
//      where F::RealType:Debug {

impl<F:Scalar,
     const DIMXV:usize,
     X:Clone+Sub<Output=XV>+Torsor+ConstElement+Tolerance,
     XV:Clone+FiniteDimensionalInnerProductSpace<F, DIMXV>,
     Y:Sub<Output=YV>+Torsor+ConstElement+Clone,
     YV:Vectorspace<F>> Transformation<F::RealType, X, Y> for AffineTransformation<F, DIMXV, X,Y>
    where F::RealType : Debug,
          X::DistT    : PartialOrd,
          XV::ScProdT : Zero+Clone+Parameter<F> {

    fn apply(&self, x:X) -> Y {
        self.apply_private(x)
    }


    fn try_approx_with_weights(orig_imag_pairs:Vec<(F::RealType,X,Y)>) -> Result<Self,ApproximationTrafoError> {
        let (affine_basis, inds)=
            AffineBasis::try_new(orig_imag_pairs.iter()
                    .map(|(_,x,_)|x.clone()))
                 .ok_or(ApproximationTrafoError::InsufficientData)?;
        let images:Vec<Y>=inds.iter().map(|&i|orig_imag_pairs[i].2.clone()).collect();
        Ok(Self{affine_basis, images})
    }

    fn defining_points() -> impl ExactSizeIterator<Item=X> {
        <X as algebra_traits::AffineBasis<F>>::affine_basis()
    }
    
    fn try_approx(orig_imag_pairs:Vec<(X,Y)>) -> Result<Self,ApproximationTrafoError> {
        Self::try_approx_with_weights(orig_imag_pairs.into_iter().map(|xy|(<F::RealType>::one(), xy.0, xy.1)).collect())
    }
    
    fn try_new(f: impl Fn(X) -> Y) -> Result<Self,ApproximationTrafoError> {
        let pts_iter=||Self::defining_points().into_iter();
        Self::try_approx_with_weights(pts_iter().zip(pts_iter()).map(|(pt0,pt1)|(<F::RealType>::one(), pt0, f(pt1))).collect())
        .map_err(|e|match e {
            // if there is insufficient data its because defining points does not contain enough points
            ApproximationTrafoError::InsufficientData => ApproximationTrafoError::DefiningPointsNotCorrect,
            err => err
            })
    }
    
    fn try_composition<Mid,
                       TLhs:Transformation<F::RealType,Mid,Y>,
                       TRhs:Transformation<F::RealType,X,Mid>>(lhs:TLhs, rhs:TRhs) -> Result<Self, ApproximationTrafoError> {
        Self::try_new(|x:X|lhs.apply(rhs.apply(x)))
    }
    
    fn images(&self) -> impl ExactSizeIterator<Item=Y> {
        Self::defining_points()
                .into_iter()
                .map(|pt|self.apply(pt))
    }
    
    fn try_inverse<T:Transformation<F::RealType,Y,X>>(&self) -> Result<T, ApproximationTrafoError> {
        T::try_approx_with_weights(
            Self::defining_points()
                                .into_iter()
                                .zip(self.images())
                                .map(|(pt,pt_img)|(<F::RealType>::one(), pt_img, pt))
                                .collect())
    }
    
    fn try_from(other:impl Transformation<F::RealType,X,Y>) -> Result<Self,ApproximationTrafoError> {
        Self::try_new(|x:X|other.apply(x))
    }
}

impl<F:Clone+Scalar,
    const DIMXV:usize,
    X:Clone+Sub<Output=XV>+Torsor+ConstElement+Tolerance,
    XV:Clone+FiniteDimensionalInnerProductSpace<F,DIMXV>> AffineTransformation<F, DIMXV, X, X> 
    where F::RealType : Debug,
          X::DistT    : PartialOrd,
          XV::ScProdT : Zero+Clone+Parameter<F> {
    pub fn from_trafo<T:Clone+Mul<X,Output=X>>(trafo:T) -> Self  {
        <Self as Transformation<F::RealType, X, X>>::try_new(|x|trafo.clone()*x.clone()).unwrap()
    }
}

impl<F:Clone+RealNumber+Debug,
     const DIMXV:usize,
     X:Clone+Sub<Output=XV>+MetricTorsor+ConstElement+Tolerance+'static,
     XV:Clone+FiniteDimensionalInnerProductSpace<F,DIMXV>>
     From<Translation<XV>> for AffineTransformation<F,DIMXV, X, X>
    where X::DistT    : Zero+Max+PartialOrd,
          XV::ScProdT : Zero+Clone+Parameter<F> {
    fn from(t:Translation<XV>) -> Self {
        <Self as Transformation<F::RealType, X, X>>::try_new(|x|t.apply(x)).unwrap()
    }
}

impl<F:Clone+RealNumber+Mul<V,Output=V>+Debug,
     A:Clone+Sub<Output=V>+MetricTorsor<DistT=NT>+ConstElement+Origin+Tolerance+'static,
     V:Clone+TryDiv<Output=F>+PartialOrd+InnerProductSpace1d<NormT=NT,Norm2T = NT2>,
     NT:num_traits::Zero+Max+Pow2<Output=NT2>+ScalarMul<F::RealType>,
     NT2:num_traits::Zero+Max+TrySqrt<Output=Nonnegative<NT>>>
     From<SE3<F,V>> for AffineTransformation<F,3, Point3<A>>
    where A::DistT   : Zero+Max+PartialOrd,
          V::ScProdT : Zero+Clone+Parameter<F> {
    fn from(se:SE3<F,V>) -> Self {
        <Self as Transformation<F::RealType, Point3<A>>>::try_new(|x|<SE3<F,V> as Mul<Point3<A>>>::mul(se.clone(), x.clone())).unwrap()
    }
}

// #[cfg(test)]
// use algebra_traits::{TryDiv,Scalar, FiniteDimensionalInnerProductSpace, InnerProductSpace1d, Max, Pow2, ScalarMul};

// #[cfg(test)]
// fn test_fin_dim_inner_prod_space<
//     V:TryDiv<Output=F>+InnerProductSpace1d<NormT=NT,Norm2T = NT2>,
//     NT:,
//     NT2:num_traits::Zero+Max,
//     F:Scalar>(v:Vector3<V>) -> impl FiniteDimensionalInnerProductSpace<F,3> {
//     v
// }

// #[test]
// fn test_from_translation() {
//     use crate::{Point3, Vector3};
//     let t=Translation::new(Vector3::new(1.0,0.0,0.0));
//     <AffineTransformation::<f64, 3, Point3<f64>, Point3<f64>> as From<Translation<Vector3<f64>>>>::from(t);
// }

impl<F:Clone+Scalar,
     const DIMXV:usize,
     X:Sub<Output=XV>+Torsor+ConstElement+Tolerance+Clone+Debug+'static,
     XV:Clone+FiniteDimensionalInnerProductSpace<F, DIMXV>,
     const DIMYV:usize,
     Y:Sub<Output=YV>+Torsor+ConstElement+Clone+Debug+'static,
     YV:Clone+FiniteDimensionalInnerProductSpace<F, DIMYV>,
     Z: Sub<Output=ZV>+Torsor+ConstElement+Clone+Debug+'static,
     ZV:Clone+InnerProductSpace<F>>
         Mul<AffineTransformation<F, DIMXV, X, Y>> for AffineTransformation<F, DIMYV, Y, Z> where
         X::DistT    : PartialOrd,
         XV::ScProdT : Clone+Zero+Parameter<F>,
         F::RealType : Debug,
        AffineTransformation<F, DIMXV, X, Y> : Transformation<F::RealType, X, Y>, 
        AffineTransformation<F, DIMYV, Y, Z> : Transformation<F::RealType, Y, Z> {
        type Output=AffineTransformation<F, DIMXV, X, Z>;
        fn mul(self, rhs: AffineTransformation<F, DIMXV, X, Y>) -> Self::Output {
            <AffineTransformation::<F,DIMXV,X,Z> as Transformation::<F::RealType,X,Z>>::try_composition(self, rhs).unwrap()
        }
}