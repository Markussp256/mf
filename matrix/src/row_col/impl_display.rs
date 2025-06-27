
// use std::fmt::Display;
// use super::*;
// use container_traits::{LinearContainer, Iter};

// use container_traits::LinearContainer;
// use super::MatrixRowGeneric;

// // impl<C:LinearContainer<T=T>,T:'static+Display>
// impl<C:LinearContainer<T=T>, T:std::fmt::Display> std::fmt::Display for MatrixRowGeneric<C> {
//     container_traits::impl_display_for_linear_container!();
// }

// Display for MatrixRowGeneric<C> {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut iter = self.iter();
//         let width=f.width().unwrap_or(8);
//         let precision= f.precision().unwrap_or(4);
//         let write_number=|f:&mut std::fmt::Formatter, t:&T|write!(f, "{:+width$.precision$}", t, width=width, precision=precision);
//         if let Some(first) = iter.next() {
//             write_number(f, first)?;
//             for value in iter {
//                 write!(f, ", ")?;
//                 write_number(f, value)?;
//             }
//         }
//         Ok(())
//     }
// }