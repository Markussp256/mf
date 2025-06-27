

// impl<F, const N:usize> ColVector for MatrixCol<F, N> {
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


// impl<F, const N:usize> ColVectorMut for MatrixCol<F,N> {
//     fn get_mut(&mut self,i:usize) ->  Option<&mut Self::T> {
//         (i<self.len()).then(||& mut self.0[i])
//     }
// }