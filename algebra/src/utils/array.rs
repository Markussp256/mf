// use num_traits::Zero;
// use std::fmt::Debug;
// use algebra_traits::{Scalarproduct, TryDiv};
// use container_traits::IntoSum;

// only functions that require algebra_traits should be here the rest should be put to utils

// we could implement own traits for the array, however we can not implements foreign traits for array (orphan rule)
// we make free functions for all traits instead  

// pub fn try_div<T:TryDiv<T2,Output=TR,Error=E>,E:Debug, T2:Zero+Clone, TR, const N:usize>(a:[T;N], b:T2) -> Option<[TR;N]> {
//     (!b.is_zero()).then(||
//         a.map(|t|t.try_div(b.clone()).unwrap()))
// }

// pub fn scalar_product<
//     T: Scalarproduct<ScProdT=TR>,
//     TR: Zero, // note: Zero also includes Add
//     const N: usize,
// >(a: [T; N],
//   b: [T; N]) -> TR {
//     a.into_iter()
//      .zip(b.into_iter())
//      .map(|(ai,bi)|ai.scalar_product(bi))
//      .into_sum()
// }


