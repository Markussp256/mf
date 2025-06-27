use std::ops::Mul;

use algebra::VectorDyn;

use matrix_traits::{TryVectorVectorProduct,try_vector_vector_product_impl};
use num_traits::Zero;

use super::*;




// impl<T> TryFromLenAndFn<T> for MatrixRowDyn<T> {
//     fn try_from_len_and_fn(len:usize, f: impl Fn(usize) -> Option<T>) -> Option<Self> {
//         let ots:Vec<Option<T>>=
//             (0..len).into_iter()
//                     .map(f)
//                     .collect();
//         let all_some=<_ as Iter>::iter(&ots).all(Option::is_some);
//         all_some.then(||
//             Self::from_iter(ots.into_iter().map(Option::unwrap)))
//     }
// }


macro_rules! row_col_prod {
    ($tr:ident, $col:ident $(,$fn:ident)? ) => {
        impl<F:Mul<F2,Output=F3>, F2, F3:Zero> $tr<$col<F2>> for MatrixRowDyn<F> {
            $(type Output=F3;
            fn $fn(self, rhs:$col<F2>) -> Option<F3> {
                try_vector_vector_product_impl(self,rhs)
                // container_traits::vec_op::try_row_col_mul(self.into_vec(),rhs.into())
            })?
        }
    };
}
row_col_prod!(TryVectorVectorProduct,  MatrixColDyn, try_vector_vector_product);
row_col_prod!(TryVectorVectorProduct,  VectorDyn,    try_vector_vector_product);



// impl<F> RowVector for MatrixRowDyn<F> {
//     type T=F;

//     fn try_from<F2:Into<Self::T>>(v:impl IntoIterator<Item=F2>) -> Result<Self,Vec<F2>> {
//         Ok(Self::from_iter(v.into_iter().map(|vi|vi.into())))
//     }

//     fn len(&self) -> usize {
//         <EnhancedVec<F> as container_traits::for_dynamic::Len>::len(&self.0)
//     }

//     fn is_len_possible(_:usize) -> bool {
//         true
//     }

//     fn get(&self,i:usize) -> Option<&Self::T> {
//         (i<self.len()).then(||&self.0[i])
//     }
    
//     fn container_into_iter(self) -> impl Iterator<Item = Self::T> {
//         self.0.into_iter()
//     }
// }

// impl<F> RowVectorMut for MatrixRowDyn<F> {
//     fn get_mut(&mut self,i:usize) ->  Option<&mut Self::T> {
//         (i<self.len()).then(||& mut self.0[i])
//     }
// }