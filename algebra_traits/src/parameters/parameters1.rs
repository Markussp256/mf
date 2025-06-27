// note that the implementation of parameter from_parameter should be constistent with the PartialOrd

pub trait Parameters1<F = f64>: super::Parameters<F>+super::NumberOfDegreesOfFreedom<F> {
    fn parameter(&self) -> F;
    fn from_parameter(f: F) -> Self;
}

// got replaced by derive macro
// unfortunately can not be made generic for every type
// #[macro_export]
// macro_rules! impl_parameters1_f_for_newtype {
//     ($name:ident) => {
//         impl<F:Clone> $crate::NumberOfDegreesOfFreedom<F> for $name<F>{
//             const NDOFS:usize=1;
//         }

//         impl<F:Clone> $crate::Parameters1<F> for $name<F> {
//             fn parameter(&self) -> F {
//                 self.0.clone()
//             }
//             fn from_parameter(f: F) -> Self {
//                 Self(f)
//             }
//         }

//         impl<F:Clone> $crate::Parameters<F> for $name<F> {
//             fn parameters(&self) -> Vec<F> {
//                 vec![self.0.clone()]
//             }
        
//             fn try_from_iter<I: Iterator<Item = F >>(iter: &mut I) -> Option<Self> {
//                 iter.next()
//                     .map(|f|Self(f))
//             }
//         }
//     };
// }


// used for impl_real_numbers
// otherwise use derive_macro
// #[macro_export]
macro_rules! impl_parameters1_self {
    ($f:ident $(<$t:ident>)?) => {
        impl $(<$t : Clone>)? $crate::NumberOfDegreesOfFreedom<$f $(<$t>)?> for $f $(<$t>)?{
            const NDOFS:usize=1;
        }

        impl $(<$t: Clone>)? $crate::Parameters1<$f $(<$t>)?> for $f $(<$t>)? {
            fn parameter(&self) -> $f $(<$t>)? {
                self.clone()
            }
            fn from_parameter(f: $f $(<$t>)?) -> Self {
                f
            }
        }

        impl $(<$t: Clone>)? $crate::Parameters<$f $(<$t>)?> for $f $(<$t>)? {
            fn parameters(&self) -> Vec<$f $(<$t>)?> {
                vec![self.clone()]
            }
        
            fn try_from_iter<I: Iterator<Item = $f $(<$t>)?  >>(iter: &mut I) -> Option<Self> {
                iter.next()
            }
        }
    };
}
pub (crate) use impl_parameters1_self;