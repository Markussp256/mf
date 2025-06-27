// we implement our own Add, Sub, Mul
// for use in array/vec because std::ops can not be used
// we dont have Div because division is dangerous (division by zero)
// use try_div instead

use container_traits::{ContainerTryConstruct, ContainerConstructError, Map};

macro_rules! neg_inv {
    ($tr:ident,$fn:ident) => {
        pub trait $tr {
            type Output;
            fn $fn(self) -> Self::Output;
        }
    };
}
neg_inv!(Neg,neg);
neg_inv!(Inv,inv);

impl<C    : ContainerTryConstruct<usize,ContainerConstructError<usize>,T=T>+Map<T,TOut,Output=COut>,
     COut : ContainerTryConstruct<usize,ContainerConstructError<usize>,T=TOut>,
     T : std::ops::Neg<Output = TOut>,
     TOut> Neg for C {
    type Output=COut;
    fn neg(self) -> Self::Output {
        self.map(<T as std::ops::Neg>::neg)
    }
}

macro_rules! def_bin_op {
    ($tr:ident,$fn:ident) => {
        pub trait $tr<RHS=Self> {
            type Output;
            fn $fn(self, rhs:RHS) -> Self::Output;
        }
    };
}
def_bin_op!(Add,add);
def_bin_op!(Sub,sub);
def_bin_op!(Mul,mul);
def_bin_op!(Div,div);

macro_rules! impl_eleme_wise_bin_op {
    ($tr:ident,$fn:ident) => {
        impl<T:std::ops::$tr<T2,Output=TR>,T2,TR,const N:usize> $tr<[T2;N]> for [T;N] {
            type Output=[TR;N];
            fn $fn(self, rhs:[T2;N]) -> [TR;N] {
                self.into_iter()
                    .zip(rhs.into_iter())
                    .map(|(l,r)|l.$fn(r))
                    .collect::<Vec<TR>>()
                    .try_into()
                    .ok().unwrap()

                // utils::iter::next_chunk(& mut self.into_iter()
                // .zip(rhs.into_iter())
                // .map(|(l,r)|l.$fn(r))).ok().unwrap()
            }
        }
    };
}
impl_eleme_wise_bin_op!(Add,add);
impl_eleme_wise_bin_op!(Sub,sub);

// scalar multiplication
impl<T:std::ops::Mul<T2,Output=TR>,T2:Clone,TR,const N:usize> Mul<T2> for [T;N] {
    type Output=[TR;N];
    fn mul(self,rhs:T2) -> Self::Output {
        self.map(|ti|ti*rhs.clone())
    }
}

impl<T:std::ops::Mul<T2,Output=TR>,T2:Clone,TR> Mul<T2> for Vec<T> {
    type Output=Vec<TR>;
    fn mul(self,rhs:T2) -> Self::Output {
        self.into_iter()
            .map(|ti|ti*rhs.clone())
            .collect()
    }
}


#[macro_export]
macro_rules! impl_binary {
    ($name:ident $(<$c:ident>)? , $tr:ident, $fn:ident) => {
        impl<$($c,)?Rhs,COut> $crate::operators::basic::$tr<Rhs> for $name$(<$c>)?
        where Self : std::ops::$tr<Rhs,Output=COut> {
            type Output=COut;
            fn $fn(self, rhs:Rhs) -> COut {
                <Self as std::ops::$tr<Rhs>>::$fn(self,rhs)
            }
        }
    }
}

// implements traits from this crate
// primarily intended for use in this crate, otherwise one can use derive macro
// container_derive::AlgOps
// #[macro_export]
// macro_rules! alg_from_ops_num {
//     ($name:ident $(<$c:ident>)? ) => {
//         $crate::alg_from_ops_num_without_inv!($name $(<$c>)?);

//         impl$(<$c>)? $crate::operators::basic::Inv for $name$(<$c>)? where Self : num_traits::Inv {
//             type Output=<Self as num_traits::Inv>::Output;
//             fn inv(self) -> Self::Output {
//                 <Self as num_traits::Inv>::inv(self)
//             }
//         }
//     }
// }

// implements traits from this crate
// primarily intended for use in this crate, otherwise one can use derive macro
// container_derive::AlgOps
// #[macro_export]
// macro_rules! alg_from_ops_num_without_inv {
//     ($name:ident $(<$c:ident>)? ) => {

//         impl$(<$c>)? $crate::scalar::zero::Zero for $name$(<$c>)? where Self : num_traits::Zero {
//             fn zero() -> Self {
//                 <Self as num_traits::Zero>::zero()
//             }
//         }

//         impl$(<$c>)? $crate::scalar::zero::IsAZero for $name$(<$c>)? where Self : num_traits::Zero {
//             fn is_a_zero(&self) -> bool {
//                 <Self as num_traits::Zero>::is_zero(&self)
//             }
//         }

//         impl$(<$c>)? $crate::scalar::one::One for $name$(<$c>)? where Self : num_traits::One {
//             fn one() -> Self {
//                 <Self as num_traits::One>::one()
//             }
//         }

//         impl$(<$c>)? $crate::operators::basic::Neg for $name$(<$c>)? where Self : std::ops::Neg {
//             type Output=<Self as std::ops::Neg>::Output;
//             fn neg(self) -> Self::Output {
//                 -self
//             }
//         }

//         $crate::impl_binary!($name$(<$c>)?,Add,add);
//         $crate::impl_binary!($name$(<$c>)?,Sub,sub);
//         $crate::impl_binary!($name$(<$c>)?,Mul,mul);
//         $crate::impl_binary!($name$(<$c>)?,Div,div);

//         impl$(<$c>)? $crate::AdditiveGroup for $name $(<$c>)?
//             where Self : std::ops::Add<Output=Self>
//                        +$crate::TryAdd<Output=Self>
//                         +std::ops::Sub<Output=Self>
//                        +$crate::TrySub<Output=Self>
//                         +std::ops::Neg<Output=Self>
//                         +num_traits::Zero {}
//     };
// }