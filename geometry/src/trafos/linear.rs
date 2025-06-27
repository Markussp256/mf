use algebra_traits::*;
use container_traits::{Parameter,Iter};
use geometry_traits::transformation::*;

use num_traits::Zero;
use vector_and_affine_spaces::Basis;

use std::{fmt::Debug, ops::Mul};

#[derive(Clone, Debug)]
pub struct ContradictingDataForLinearTransformation<F:Scalar, const DIMX:usize, X, Y> {
    pub ws     : Vec<F>,
    pub basis  : Basis<F, X, DIMX>,
    pub images : Vec<Y>,
    pub x      : X,
    pub y      : Y,
    pub yalt   : Y,
    pub d      : Nonnegative<F::RealType>,
    pub tol    : Nonnegative<F::RealType>
}

impl<const DIMX:usize,
     F     : Scalar+Clone+Debug,
     X     : FiniteDimensionalInnerProductSpace<F, DIMX, NormT=NormT>+Clone+Debug,
     NormT : Zero+Pow2+PartialOrd,
     Y     : Vectorspace<F>+Debug> ContradictingDataForLinearTransformation<F, DIMX, X, Y> {

    pub fn lin_comb(&self, f:impl Fn(usize) -> String) -> String {
        let coeffs=self.ws.clone();
        let f=|i:usize|format!("{:?}*{:?}",coeffs[i],f(i));
        let mut string=f(0);
        for i in 1..coeffs.len() {
            string+=&format!("+{:?}",f(i));
        }
        string
    }

    pub fn lin_combs(&self) -> String {
        let basis:&[X;DIMX]=self.basis
                              .basis();
        let x=|i:usize| format!("{:?}",basis[i]);
        let fx=|i:usize| format!("f({:?})",basis[i]);
        let y=|i:usize| format!("{:?}",self.images[i]);
        format!("f({:?})={:?}={:?}",self.lin_comb(x), self.lin_comb(fx), self.lin_comb(y))
    }
}

impl<const DIMX:usize,
     F     : Scalar+Clone+Debug,
     X     : FiniteDimensionalInnerProductSpace<F,DIMX,NormT=NormT>+Clone+Debug,
     NormT : Zero+Pow2+PartialOrd,
     Y     : Vectorspace<F>+Debug> std::fmt::Display for ContradictingDataForLinearTransformation<F, DIMX, X,Y> 
     where F::RealType : Debug {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"Contradicting data to estimate Linear transformation: we have 
        {:?}=f({:?})={:?}={:?} but {:?}.distance({:?})={:?}>tol={:?}",
        self.y, self.x, self.lin_combs(), self.yalt, self.y, self.yalt, self.d, self.tol)
    }
}

impl<F:Scalar+Clone+Debug,
     const DIMX:usize,
     X:FiniteDimensionalInnerProductSpace<F, DIMX, NormT=NormT>+Clone+Debug,
     NormT:Zero+Pow2+PartialOrd,
     Y:                 Vectorspace<F>+               Debug> std::error::Error
     for ContradictingDataForLinearTransformation<F, DIMX, X,Y> 
     where F::RealType : Debug {}


impl<F:Scalar+Clone+Debug,
     const DIMX:usize,
     X:FiniteDimensionalInnerProductSpace<F, DIMX, NormT=NormT>+Clone+Debug,
     NormT:Zero+Pow2+PartialOrd,
     Y:                 Vectorspace<F>+               Debug> ContradictingDataForApproximatingTrafoError
     for ContradictingDataForLinearTransformation<F, DIMX, X,Y> 
     where F::RealType : Debug {}

#[derive(Clone, Debug)]
pub struct LinearTransformation<
    F : Scalar,
    const DIMX:usize,
    X : FiniteDimensionalVectorspace<F, DIMX>,
    Y : Vectorspace<F>> {
    basis  : Basis<F, X, DIMX>,
    images : Vec<Y>
}

impl<const DIMX:usize,
     F : Scalar,
     X : FiniteDimensionalVectorspace<F, DIMX>,
     Y : Vectorspace<F>> LinearTransformation<F, DIMX, X,Y> {

    pub fn into_parts(self) -> (Basis<F,X,DIMX>, Vec<Y>) {
        (self.basis, self.images)
    }
}

impl<const DIMX:usize,
     F : Scalar+Clone+Debug+'static,
     X : FiniteDimensionalInnerProductSpace<F,DIMX>+Clone+Debug+'static+Tolerance,
     Y : Vectorspace<F>+Clone+Debug+'static> Transformation<F::RealType, X, Y>
     for LinearTransformation<F, DIMX, X, Y>
    where X::DistT : PartialOrd,
        X::ScProdT : Clone+Zero+Parameter<F> {
    
    fn apply(&self, v:X) -> Y {
        let (basis,images)=self.clone().into_parts();
        let coords=basis.clone().find_coordinates(v);
        Y::linear_combination(coords.into_iter().zip(images))
    }

    fn try_approx_with_weights(orig_imag_pairs:Vec<(F::RealType,X,Y)>) -> Result<Self,ApproximationTrafoError> {
        let (basis, inds)=
            Basis::try_new(
                orig_imag_pairs.iter()
                                   .map(|(_,x,_)|x.clone()))
            .ok_or(ApproximationTrafoError::InsufficientData)?;
        let images:Vec<Y>=inds.iter().map(|&i|orig_imag_pairs[i].clone().2).collect();
        // for window in inds.windows(2) {
        //     if let [lb,ub]=window {
        //         for i in lb+1..ub.clone() {
        //             let (_,x,y)=orig_imag_pairs[i].clone();
        //             let c=basis.find_coordinates(&x);
        //             let yalt=Y::linear_combination(c.iter().zip(images.iter()).map(|(ci,ii)|(ci.clone(),ii.clone())).collect());
        //             let d=yalt.distance(&y);
        //             if d > tol {
        //                 let cdat=ContradictingDataForLinearTransformation{
        //                     ws:c, basis, images, x, y, yalt, d, tol
        //                 };
        //                 return Err(ApproximationTrafoError::ContradictingData(Box::new(cdat)));
        //             }
        //         }
        //     }
        // }
        Ok(Self{basis, images})
    }

    fn defining_points() -> impl ExactSizeIterator<Item=X> {
        X::basis()
    }
}

impl<F:Scalar+Clone+Debug+'static,
     const DIMX:usize,
     X:FiniteDimensionalInnerProductSpace<F, DIMX>+Clone+Debug+'static+Tolerance,
    const DIMY:usize,
     Y:FiniteDimensionalInnerProductSpace<F, DIMY>+Clone+Debug+'static+Tolerance,
     Z:Vectorspace<F>+Clone+Debug+'static> Mul<LinearTransformation<F, DIMX, X, Y>>
     for LinearTransformation<F, DIMY, Y, Z>
    where X::DistT:PartialOrd,
          X::ScProdT : Clone+Zero+Parameter<F>,
          Y::DistT:PartialOrd,
          Y::ScProdT : Clone+Zero+Parameter<F> {
        type Output=LinearTransformation<F, DIMX, X, Z>;
        fn mul(self, rhs: LinearTransformation<F, DIMX, X, Y>) -> Self::Output {
            Self::Output::try_composition(self, rhs).unwrap()
        }
}