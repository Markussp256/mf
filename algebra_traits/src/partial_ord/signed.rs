// use std::ops::Neg;

// use num_traits::Zero;

// #[derive(PartialEq, Eq)]
// pub enum Sign {
//     PlusOne,
//     MinusOne,
//     Zero,
//     NaN
// }


// pub trait Signed : Zero+Neg<Output=Self>+Clone {
//     // required methods
//     fn is_positive(&self) -> bool;

//     // provided methods
//     fn is_negative(&self) -> bool {
//         (-self.clone()).is_positive()
//     }

//     fn sign(&self) -> Sign {
//         if self.is_positive() {
//             Sign::PlusOne
//         } else if self.is_negative() {
//             Sign::MinusOne
//         } else if self.is_zero() {
//             Sign::Zero
//         } else {
//             Sign::NaN
//         }
//     }

//     fn abs(self) -> Self {
//         if self.is_negative() {
//             -self
//         } else {
//             self
//         }
//     }


    
//     // might not be correct for NaN, but I dont care about NaN
//     fn is_non_negative(&self) -> bool {
//         self.is_positive() || self.is_zero()
//     }

//     fn is_non_positive(&self) -> bool {
//         self.is_negative() || self.is_zero()
//     }
// }

// impl Signed for f64 {
//     fn is_positive(&self) -> bool {
//         self > &0.0
//     }
// }

// impl Signed for i64 {
//     fn is_positive(&self) -> bool {
//         i64::is_positive(self.clone())
//     }
// }