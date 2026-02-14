// use num_traits::Zero;
// use container_traits::IntoSum;
// use std::ops::Mul;

// use crate::row_col::{ColVector, RowVector};

use std::ops::Mul;
use container_traits::IntoSum;
use num_traits::Zero;

use crate::{ColVector, RowVector, ColVectorView, RowVectorView};


// impl code using only method from Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_into_vector_vector_product_impl
    <F:Mul<F2,Output=F3>,
     F2,
     F3:Zero,
     Lhs:RowVector<T=F>,
     Rhs:ColVector<T=F2>>(lhs:Lhs,rhs:Rhs) -> Option<F3> {
        (lhs.len() == rhs.len()).then(||
              lhs.into_iterator()
                .zip(rhs.into_iterator())
                .map(|(ai,bi)|ai*bi)
                .into_sum())
}

pub fn try_vector_vector_product_impl
    <F  : Clone+Mul<F2,Output=F3>,
     F2 : Clone,
     F3 : Zero,
     Lhs:RowVectorView<T=F>,
     Rhs:ColVectorView<T=F2>>(lhs:&Lhs,rhs:&Rhs) -> Option<F3> {
        (lhs.len() == rhs.len()).then(||
              lhs.iter().cloned()
                .zip(rhs.iter().cloned())
                .map(|(ai,bi)|ai*bi)
                .into_sum())
}


pub trait VectorVectorProduct<Rhs : ColVectorView> : RowVectorView {
    type Output;
    fn vector_vector_product(&self, rhs:&Rhs) -> <Self as VectorVectorProduct<Rhs>>::Output;
}

pub trait IntoVectorVectorProduct<Rhs : ColVectorView> : RowVectorView {
    type Output;
    fn into_vector_vector_product(self, rhs:Rhs) -> <Self as IntoVectorVectorProduct<Rhs>>::Output;
}

pub trait TryVectorVectorProduct<Rhs : ColVectorView> : RowVectorView {
    type Output;
    fn try_vector_vector_product(&self, rhs:&Rhs) -> Option<<Self as TryVectorVectorProduct<Rhs>>::Output>;
}


pub trait TryIntoVectorVectorProduct<Rhs : ColVectorView> : RowVectorView {
    type Output;
    fn try_into_vector_vector_product(self, rhs:Rhs) -> Option<<Self as TryIntoVectorVectorProduct<Rhs>>::Output>;
}