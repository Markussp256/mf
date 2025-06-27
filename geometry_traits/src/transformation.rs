// general map X -> Y

use thiserror::Error;

use num_traits::{Zero, One};


pub trait Transform<Trafo> {
    type Output;
    fn transform(&self, trafo:Trafo) -> Self::Output;
}


pub trait ContradictingDataForApproximatingTrafoError : std::error::Error {}

#[derive(Error, Debug)]
pub enum ApproximationTrafoError {
    #[error("not enough source-image pairs to estimate the transformation")]
    InsufficientData,

    #[error("the points in the overloaded defining_points function and its images do not seem to be enough to define the transformation")]
    DefiningPointsNotCorrect,

    #[error("the sum of the weights is not allowed to be zero or smaller")]
    SumOfWeightsNonpositive,

    // #[error("could not find a transformation that fits the pairs to the tolerance {0}")]
    // CouldNotFindTrafoForTol(f64),

    // #[error("inconsistent data")]
    // ContradictingData(Box<dyn ContradictingDataForApproximatingTrafoError>)
}


pub trait Transformation<W:Zero+One, X, Y=X> : Sized {

    // required methods
    fn apply(&self, x:X) -> Y;

    // find approximation that best fits the pairs with the transformation reports an error  
    fn try_approx_with_weights(orig_imag_pairs:Vec<(W,X,Y)>) -> Result<Self, ApproximationTrafoError>;

    // list of points such that the pairs of points and its images define the transformation
    fn defining_points() -> impl ExactSizeIterator<Item=X>;

    // provided methods
    fn try_approx(orig_imag_pairs:Vec<(X,Y)>) -> Result<Self,ApproximationTrafoError> {
        Self::try_approx_with_weights(orig_imag_pairs.into_iter().map(|xy|(W::one(), xy.0, xy.1)).collect())
    }

    fn try_new(f: impl Fn(X) -> Y) -> Result<Self,ApproximationTrafoError> {
        let pts_iter=||Self::defining_points().into_iter();
        Self::try_approx_with_weights(pts_iter().zip(pts_iter()).map(|(pt0,pt1)|(W::one(), pt0, f(pt1))).collect())
        .map_err(|e|match e {
            // if there is insufficient data its because defining points does not contain enough points
            ApproximationTrafoError::InsufficientData => ApproximationTrafoError::DefiningPointsNotCorrect,
            err => err
            })
    }

    fn try_composition<
        Mid,
        TLhs : Transformation<W,Mid,Y>,
        TRhs : Transformation<W,X,  Mid>>(lhs:TLhs, rhs:TRhs) -> Result<Self, ApproximationTrafoError> {
        Self::try_new(|x:X|lhs.apply(rhs.apply(x)))
    }

    fn images(&self) -> impl ExactSizeIterator<Item=Y> {
        Self::defining_points()
                .into_iter()
                .map(|pt|self.apply(pt))
    }

    fn try_inverse<T:Transformation<W,Y,X>>(&self) -> Result<T, ApproximationTrafoError> {
        T::try_approx_with_weights(
            Self::defining_points()
                                .into_iter()
                                .zip(self.images())
                                .map(|(pt,pt_img)|(W::one(), pt_img, pt))
                                .collect())
    }

    fn try_from(other:impl Transformation<W,X,Y>) -> Result<Self,ApproximationTrafoError> {
        Self::try_new(|x:X|other.apply(x))
    }
}

// impl<X,Y, T1:Transformation<X=X,Y=Y>, T2:Transformation<X=X,Y=Y>> TryInto<T2> for T1 {
//     type Error=ApproximationTrafoError;

//     fn try_into(self) -> Result<T2, Self::Error> {
//         T2::try_new(|x|self*x, None)
//     }
// }

// impl<T:Transformation> std::ops::Mul<T::X> for T {
//     type Output=T::Y;

//     fn mul(self, rhs: X) -> Self::Output {
//         self.apply(rhs)
//     }
// }