use crate::operators::basic::Sub;
use crate::{TrySub, Nonnegative, Norm};

// pub trait AnyDistance : Sized {
//     type AnyDistT; // :Zero+Max

//     fn any_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<Self::AnyDistT>,SubError>;
// }

// macro_rules! impl_any_dist {
//     ($t:ty $(, const $n:ident :usize)? ) => {
//         impl<T, $(const $n:usize,)? S,NormT> AnyDistance for $t
//             where Self : TrySub<Output=S>,
//                    S    : Norm<NormT=NormT> {
//             type AnyDistT = NormT;
//             fn any_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<Self::AnyDistT>,SubError> {
//                 let rhs:Self=rhs.into();
//                 rhs.try_sub(self)
//                    .map(|d|d.norm())
//             }
//         }
//     }
// }
// impl_any_dist!(Vec<T>);
// impl_any_dist!([T;N], const N:usize);

// impl<T,S,NormT> AnyDistance for Vec<T>
//     where Self : TrySub<Output=S>,
//           S    : Norm<NormT=NormT> {
//         type AnyDistT = NormT;
//     impl_any_dist!();
// }
// impl<T,S,NormT, const N:usize> AnyDistance for [T;N]
//     where Self : TrySub<Output=S>,
//           S    : Norm<NormT=NormT> {
//     type AnyDistT = NormT;
//     impl_any_dist!();
// }




// Distance
pub trait Distance : Sized {
    type DistT;
    fn distance(self, rhs:impl Into<Self>) -> Nonnegative<Self::DistT>;
}
// we can not use std::ops::Sub on array
impl<T, const N:usize, S, NormT> Distance for [T;N] where Self : Sub<Output=S>, S: Norm<NormT=NormT> {
    type DistT=NormT;
    fn distance(self, rhs:impl Into<Self>) -> Nonnegative<Self::DistT> {
        let rhs:Self=rhs.into();
        rhs.sub(self)
           .norm()
    }
}


// #[macro_export]
// macro_rules! impl_distance_from_sub_norm {
//     ($name:ident $(<$t:ident  $(,$n:ident)* >)? ) => {
//         impl<SD, SO : num_traits::Zero+$crate::Max  $(,$t $(,const $n:usize)*)?> $crate::Distance for $name $(<$t $(,$n)*>)?
//         where Self : std::ops::Sub<Output=SD>,
//               SD : $crate::Norm<NormT=SO> {
//             type DistT = SO;
//             fn distance(self, rhs:impl Into<Self>) -> $crate::Nonnegative<SO> {
//                 let rhs:Self=rhs.into();
//                 <SD as $crate::Norm>::norm(rhs-self)
//             }
//         }
//     };
// }



// TryDistance
pub trait TryDistance : Sized {
    type TryDistT;
    type Error;
    fn try_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<Self::TryDistT>,Self::Error>;
}

impl<T:TrySub<Output=TD,Error=E>, E, TD, NormT, const N:usize> TryDistance for [T;N] where [TD;N] : Norm<NormT=NormT> {
    type TryDistT=NormT;
    type Error=E;
    fn try_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<NormT>,E> {
        let rhs:Self=rhs.into();
        rhs.try_sub(self)
           .map(Norm::norm)
    }
}

impl<T, S, E, NormT> TryDistance for Vec<T> where Self : TrySub<Output=S,Error=E>, S : Norm<NormT=NormT> {
    type TryDistT=NormT;
    type Error=E;
    fn try_distance(self, rhs:impl Into<Self>) -> Result<Nonnegative<NormT>,E> {
        let rhs:Self=rhs.into();
        rhs.try_sub(self)
           .map(|d|d.norm())
    }
}


