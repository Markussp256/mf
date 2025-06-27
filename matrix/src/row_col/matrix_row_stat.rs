
use matrix_traits::{VectorVectorProduct,try_vector_vector_product_impl};

use super::*;

use std::ops::Mul;
use algebra::Vector;
use num_traits::Zero;

// impl<F:Mul<F2,Output=F3>,
//     F2,
//     F3:Zero,
//     const D:usize> TryVectorVectorProduct<MatrixCol<F2,D>> for MatrixRow<F,D> {}

// impl<F:Mul<F2,Output=F3>,
//     F2,
//     F3:Zero,
//     const D:usize> VectorVectorProduct<MatrixCol<F2,D>> for MatrixRow<F,D> {}


// impl<T, const N:usize> TryFrom<MatrixRowDyn<T>> for MatrixRow<T, N> {
//     type Error = MatrixRowDyn<T>;
//     fn try_from(value: MatrixRowDyn<T>) -> Result<Self, Self::Error> {
//         let oa=<MatrixRowDyn<T> as TryInto<[T;N]>>::try_into(value);
//         oa.map(|a|a.into())  
//     }
// }



macro_rules! row_col_prod {
    ($tr:ident, $col:ident $(,$fn:ident)? ) => {
        impl<T:Mul<T2, Output=TR>, T2, TR:Zero, const N:usize> $tr<$col<T2,N>> for MatrixRow<T,N> {
            $(type Output=TR;
            fn $fn(self, rhs:$col<T2, N>) -> TR {
                try_vector_vector_product_impl(self,rhs).unwrap()
            })?
        }
    };
}
row_col_prod!(VectorVectorProduct, MatrixCol, vector_vector_product);
row_col_prod!(VectorVectorProduct, Vector, vector_vector_product);


// impl<F, const N:usize> RowVector for MatrixRow<F, N> {
//     type T=F;

//     fn try_from<F2:Into<Self::T>>(v:impl IntoIterator<Item=F2>) -> Result<Self,Vec<F2>> {
//         let v:Vec<F2>=v.into_iter().collect();
//         if v.len() != N {
//             return Err(v);
//         }
//         let v:Vec<F>=v.map(|vi|vi.into());
//         Ok(v.try_into()
//             .ok()
//             .unwrap())
//     }

//     fn len(&self) -> usize {
//         N
//     }

//     fn is_len_possible(len:usize) -> bool {
//         len == N
//     }

//     fn get(&self,i:usize) -> Option<&Self::T> {
//         (i<self.len()).then(||&self.0[i])
//     }
    
//     fn container_into_iter(self) -> impl Iterator<Item = Self::T> {
//         self.0
//             .into_iter()
//     }
// }

// impl<F, const N:usize> crate::traits::RowVectorMut for MatrixRow<F,N> {
//     fn get_mut(&mut self,i:usize) ->  Option<&mut Self::T> {
//         (i<self.len()).then(||& mut self.0[i])
//     }
// }