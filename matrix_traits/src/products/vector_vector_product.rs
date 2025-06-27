// use num_traits::Zero;
// use container_traits::IntoSum;
// use std::ops::Mul;

// use crate::row_col::{ColVector, RowVector};

use std::ops::Mul;
use container_traits::IntoSum;
use num_traits::Zero;

use crate::{ColVector, RowVector};


// impl code using only method from Rowvector/Colvector traits
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_vector_vector_product_impl
    <F:Mul<F2,Output=F3>,
     F2,
     F3:Zero,
     Lhs:RowVector<T=F>,
     Rhs:ColVector<T=F2>>(lhs:Lhs,rhs:Rhs) -> Option<F3> {
        (lhs.len() == rhs.len()).then(||
              lhs.into_vec().into_iter()
                .zip(rhs.into_vec().into_iter())
                .map(|(ai,bi)|ai*bi)
                .into_sum())
}

pub trait VectorVectorProduct<Rhs> {
    type Output;
    fn vector_vector_product(self, rhs:Rhs) -> Self::Output;
}

pub trait TryVectorVectorProduct<Rhs> {
    type Output;
    fn try_vector_vector_product(self, rhs:Rhs) -> Option<Self::Output>;
}


pub trait AnyVectorVectorProduct<Rhs> {
    type Output;
    fn any_vector_vector_product(self, rhs: Rhs) -> Option<Self::Output>;
}

impl<Row: RowVector<T=T>,
     Rhs: ColVector<T=T2>,
     T:Mul<T2,Output=TR>,
     T2,
     TR:Zero> AnyVectorVectorProduct<Rhs> for Row {
    type Output=TR;
        fn any_vector_vector_product(self, rhs:Rhs) -> Option<TR> {
            try_vector_vector_product_impl(self,rhs)
    }
}
// macro_rules! row_col_vector {
//     ($lhs:ident, $rhs:ident) => {
//         $lhs.into_iter()
//                 .zip($rhs.into_iter())
//                 .map(|(ai,bi)|
//                    ai*bi)
//                 .into_sum()
//     };
// }

// impl<F:Mul<F2,Output=F3>,F2,F3:Zero,Rhs:ColVector<T=F2>,Row:RowVector<T=F>> VectorVectorProduct<Rhs> for Row {
//     type Output=F3;
//     fn vector_vector_product(self, rhs:Rhs) -> F3 {
//         assert_eq!(self.len(), rhs.len());
//         row_col_vector!(self,rhs)
//     }
// }

// impl<F:Mul<F2,Output=F3>,F2,F3:Zero,Rhs:ColVector<T=F2>,Row:RowVector<T=F>> TryVectorVectorProduct<Rhs> for Row {
//     type Output=F3;
//     fn try_vector_vector_product(self, rhs:Rhs) -> Option<F3> {
//         (self.len() == rhs.len()).then(||
//             row_col_vector!(self,rhs))
//     }
// }
