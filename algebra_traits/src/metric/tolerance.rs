use crate::{Distance, Nonnegative};

use num_traits::{Zero, One};


pub trait Tolerance : Distance
    where Self::DistT : PartialOrd {
    const THRESHOLD : Self::DistT;

    // provided method

    fn is_smaller_than(self, threshold:Self::DistT) -> bool where Self : Zero {
        self.distance(Self::zero()) < threshold
    }

    fn is_close_to(self, rhs:impl Into<Self>) -> bool {
        self.distance(rhs) < Self::THRESHOLD
    }

    fn is_small(self) -> bool where Self : Zero {
        self.is_close_to(Self::zero())
    }

    fn is_close_to_zero(self) -> bool where Self : Zero {
        self.is_small()
    }

    fn is_close_to_one(self) -> bool where Self : One {
        self.is_close_to(Self::one())
    }
}

// Self can be Signed or Nonnegative

impl<T:Tolerance> Tolerance for Nonnegative<T> where T::DistT : PartialOrd {
    const THRESHOLD:T::DistT=T::THRESHOLD;
}

// pub trait Tolerance4Complex<R:PartialOrd+Tolerance<SignedType = R>> : Distance<SignedOutput = R> {
    
//         // provided method
//         fn threshold() -> Nonnegative<R> {
//             <R as Tolerance>::threshold()
//         }

//         fn signed_threshold() -> R {
//             <R as Tolerance>::signed_threshold()
//         }

//         fn is_close_to(self, rhs:impl Into<Self>) -> bool {
//             self.distance(rhs) < Self::signed_threshold()
//         }

//         fn is_small(self) -> bool where Self:Zero {
//             Self::is_close_to(self, Self::zero())
//         }
    
//         fn is_close_to_one(self) -> bool where Self:One {
//             self.is_close_to(Self::one())
//         }
// }