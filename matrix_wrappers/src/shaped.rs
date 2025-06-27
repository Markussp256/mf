
pub mod square;
pub use square::Square;

pub mod not_tall;
pub use not_tall::NotTall;

pub mod not_wide;
pub use not_wide::NotWide;

pub mod tall;
pub use tall::Tall;

pub mod wide;
pub use wide::Wide;


// use matrix_traits::{AsMatrix, IntoMatrix, Matrix, MatrixSquare};
// use container_traits::IntoInner;

// macro_rules! impl_matrix_square {
//     ($t0:ident, $t1:ident) => {
//         impl<M:Matrix> AsMatrix for $t0<$t1<M>> {
//             type Output=M;
//             fn matrix(&self) -> &Self::Output {
//                 self.as_ref()
//                     .as_ref()
//             }
//         }

//         impl<M:Matrix> IntoMatrix for $t0<$t1<M>> {
//             type Output=M;
//             fn into_matrix(self) -> Self::Output {
//                 self.into_inner()
//                     .into_inner()
//             }
//         }
//         impl<M:Matrix> MatrixSquare for $t0<$t1<M>> {}

//     };
// }
// impl_matrix_square!(UnTall,UnWide);
// impl_matrix_square!(UnWide,UnTall);


// pub enum MatrixShape {
//     Square,
//     Tall,
//     Wide,
// }

// impl MatrixShape {
//     pub fn from_nrow_ncol(nrow:usize, ncol:usize) -> Self {
//         if nrow > ncol {
//             Self::Tall
//         } else if nrow < ncol {
//             Self::Wide
//         } else {
//             Self::Square    
//         }
//     }
// }