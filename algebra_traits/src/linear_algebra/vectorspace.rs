use container_traits::for_static::TryPutAt;
use container_traits::{IntoSum, Parameter};
use utils::iter::IntoExactSizeIterator;
use crate::metric::distance::TryDistance;
use crate::*;

pub trait Vectorspace<F> :
    AdditiveGroup 
    + ScalarMul   <F>
    + TryScalarDiv<F,Error=DivError> {
    fn linear_combination(vs:impl IntoIterator<Item=(F,Self)>) -> Self {
        vs.into_iter()
          .map(|(w,v)|v.scalar_mul(&w))
          .into_sum()
    }

    fn any_linear_combination(
        fs:impl IntoIterator<Item=F>,
        vs:impl IntoIterator<Item=Self>) -> Option<Self> {
        let mut same_length=true;
        let mut fs_iter=fs.into_iter();
        let mut vs_iter=vs.into_iter();
        let f_iter=||
        match (fs_iter.next(),vs_iter.next()) {
            (Some(f),Some(v)) => Some((f,v)),
            (None,None) => None,
            _ => { same_length=false; None}
        };
        let res=Self::linear_combination(std::iter::from_fn(f_iter));
        same_length.then_some(res)
    }
}
impl<F,
     V : AdditiveGroup
        +ScalarMul   <F>
        +TryScalarDiv<F,Error=DivError>> Vectorspace<F> for V {}


pub trait Basis<F> : Sized {
    fn basis() -> impl ExactSizeIterator<Item=Self>;
}

impl<T:num_traits::Zero+Basis<F>,F,const N:usize> Basis<F> for [T;N] {
    fn basis() -> impl ExactSizeIterator<Item=Self> {
        let dim_t=T::basis().count();
        (0..N).into_iter()
              .map(|i|T::basis().into_iter().map(move |t|<[T;N] as TryPutAt<usize,T>>::try_put_at(i,t).unwrap()))
              .flatten()
              .into_exact_size_iter(N*dim_t)
    }
}

// we can not put DIM as associated constant because we can not force DIM to be a certain constant when
// doing a bound with FiniteDimensionalVectorspace

pub trait FiniteDimensionalVectorspace<F, const DIM:usize>
    : Vectorspace<F> + Basis<F> {}

// we have extra treatement for 1dimensional vectorspace
pub trait Vectorspace1d : FiniteDimensionalVectorspace<<Self as TryDiv>::Output,1>
                          +ConstNonZero
                          +Parameter<<Self as TryDiv>::Output>
                          +TryDiv<Error=DivError>
                          +TryDiv<<Self as TryDiv>::Output, Output=Self, Error=DivError> {
    // finds coefficients f0, f1, not both zero, such that \sum_i f[i]*vs[i]=0
    // unlike Div on v this function does never panic
    fn zero_linear_combination(vs:[Self;2]) -> [<Self as TryDiv>::Output;2] {
        let [vs0,vs1]=vs;
        [vs1, -vs0].map(|vi| vi.div_nz())
    }
}
impl<F,
     V : TryDiv<Output=F,Error=DivError>
        +TryDiv<F,Output=V, Error=DivError>
        +FiniteDimensionalVectorspace<F,1>
        +ConstNonZero
        +Parameter<F>> Vectorspace1d for V{}

// pub trait NormedVectorspace1d : NormedSpace<SO=Nonnegative<Self>>
//                                +Vectorspace1d {}


// pub trait NormedVectorspace<F> : Vectorspace<F>
//                                 +NormedSpace {}


// pub trait FiniteDimensionalNormedVectorspace<F, const DIM:usize>
//     : FiniteDimensionalVectorspace<F, DIM>
//      +NormedVectorspace<F> { }

// scalar product of a vector with itself should be the same as normsquared
// however we can only state that the types can be converted

// Neg is the only trait in anyinnerproduct which is not defined in this crate

pub trait TryInnerProductSpaceWithoutNeg<F>
        : IsAZero
         +ClosedTryAdd
         +ClosedTrySub
         +ScalarMul<F>
         +TryScalarDiv<F,Error=DivError>
         +Norm
         +NormSquared
         +TryDistance<TryDistT=Self::NormT>
         +TryScalarproduct {}

impl<F,
     V : IsAZero
        +ClosedTryAdd
        +ClosedTrySub
        +ScalarMul<F>
        +TryScalarDiv<F,Error=DivError>
        +Norm
        +NormSquared
        +TryDistance<TryDistT=V::NormT>
        +TryScalarproduct> TryInnerProductSpaceWithoutNeg<F> for V {}

pub trait TryInnerProductSpace<F>
        : TryInnerProductSpaceWithoutNeg<F>
         +std::ops::Neg<Output=Self> {}

impl<F,
     V : TryInnerProductSpaceWithoutNeg<F>
        +std::ops::Neg<Output=V>> TryInnerProductSpace<F> for V {}


// pub trait TryInnerProductSpace<F>
//         : TryInnerProductSpace<F>
//          +TryAdd<Output=Self>
//          +TrySub<Output=Self>
//          +TryDistance<DistT=Self::NormT>
//          +TryScalarproduct {}

// impl<F,
//      V : TryInnerProductSpace<F>
//          +TryAdd<Output=V>
//          +TrySub<Output=V>
//          +TryDistance<DistT=V::NormT>
//          +TryScalarproduct> TryInnerProductSpace<F> for V {}

pub trait InnerProductSpace<F>
        : Vectorspace<F>
         +TryInnerProductSpace<F>
         +Distance<DistT=Self::NormT>
         +Scalarproduct {}

impl<F,
     V : Vectorspace<F>
        +TryInnerProductSpace<F>
        +Distance<DistT=V::NormT>
        +Scalarproduct> InnerProductSpace<F> for V {}

pub trait InnerProductSpace1d
        : Vectorspace1d
         +InnerProductSpace<<Self as TryDiv>::Output> {}

impl<ScProdT,
     F,
     V : TryDiv<Output=F,Error=DivError>
        +Vectorspace1d
        +InnerProductSpace<F,ScProdT = ScProdT>> InnerProductSpace1d for V {}


pub trait FiniteDimensionalInnerProductSpace<F, const DIM:usize>
             : InnerProductSpace<F>
              +FiniteDimensionalVectorspace<F, DIM> {}
impl<F,const DIM:usize,V:InnerProductSpace<F>+FiniteDimensionalVectorspace<F,DIM>> FiniteDimensionalInnerProductSpace<F,DIM> for V {}


        // where Self::NormT : Pow2+Tolerance {}

// Note that we can also Multiply Field from the left with Self, i.e
// Self::Field : std::ops::Mul<Self,Output=Self>
// which should give the same result as if we mulitply it from the right,
// scalar multiplication is commutative.
// However, if we state that, we need to declare it whenever we use Vectorspace
// therefore we dont state it here but implement it in macros for new types.