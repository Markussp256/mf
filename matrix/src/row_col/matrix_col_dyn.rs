// use algebra::{UnitVectorDyn, VectorDyn};

// use crate::traits::*;

// use super::{MatrixRowDyn,MatrixCol};

// impl<F> ColVector for MatrixColDyn<F> {
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
//          self.0.into_iter()
//     }
// }


// impl<F> ColVectorMut for MatrixColDyn<F> {
//     fn get_mut(&mut self,i:usize) ->  Option<&mut Self::T> {
//         (i<self.len()).then(||& mut self.0[i])
//     }
// }