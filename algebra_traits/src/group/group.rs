// we use IntoSum, IntoProduct instead
// macro_rules! group_chain {
//     ($op_name:ident, $fn:ident, $id_name:ident) => {
//         fn $op_name<I:Iterator<Item=Self>>(mut iter:I) -> Self {
//             match iter.next() {
//                 None => Self::$id_name(),
//                 Some(g) => g.$fn(Self::$op_name(iter))
//             }
//         }
//     };
// }

// use crate::operators::basic::*;
// use crate::scalar::{zero::Zero,one::One};

use num_traits::{Zero,One};
use crate::{ClosedAdd,ClosedSub,ClosedMul,ClosedDiv,ClosedNeg,ClosedInv};

// AdditiveGroup contains the std::ops traits so we can use +,-
pub trait AdditiveGroup :
    Zero
    +ClosedAdd // would also be included by Zero but we add it for completeness anyway
    +ClosedSub
    +ClosedNeg
{
}

impl<T : Zero
        +ClosedAdd
        +ClosedSub
        +ClosedNeg> AdditiveGroup for T {}

// impl<G:AdditiveGroup, I:Iterator<Item=G>> std::iter::Sum for I {
//     fn sum(iter: I) -> Self {
//         match iter.next() {
//             None => G::zero(),
//             Some(g) => g+iter.sum()
//         }
//     }
// }  

// AdditiveGroup contains the std::ops traits so we can use *,/
pub trait MultiplicativeGroup :
     One
    + ClosedInv
    + ClosedMul // would also be included by One but we add it for completeness anyway
    + ClosedDiv
{
}

impl<T : One
        +ClosedInv
        +ClosedMul
        +ClosedDiv> MultiplicativeGroup for T {}

// impl<G:MultiplicativeGroup, I:Iterator<Item=G>> std::iter::Product for I {
//     fn product(iter: I) -> Self {
//         match iter.next() {
//             None => G::one(),
//             Some(g) => g*iter.product()
//         }
//     }
// }