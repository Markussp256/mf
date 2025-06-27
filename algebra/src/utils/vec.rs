// use algebra_traits::TryDiv;
// use num_traits::Zero;

// only operations that require algebra_traits should be put here
// the rest can be put into utils

// pub fn try_div<T:TryDiv<T2,Output=T3>, T2:Zero+Clone, T3>(vs:Vec<T>, div:T2) -> Option<Vec<T3>> {
//     (!div.is_zero()).then(||vs.into_iter()
//                               .map(|t|t.try_div(div.clone()).unwrap())
//                               .collect())
// }