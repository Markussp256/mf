// base trait for all kinds of products
// vector vector
// matrix vector
// matrix matrix
// the idea is to define for types defined in our crates
// std::ops::Mul<Rhs> whenever GeneralizedMatrixProduct<Rhs> is valid

// pub trait GeneralizedMatrixProduct<Rhs> : Sized {
//     type Output;
//     fn generalized_matrix_product(self, rhs:Rhs) -> Self::Output;
// }

// pub trait TryGeneralizedMatrixProduct<Rhs> : Sized {
//     type Output;
//     fn try_generalized_matrix_product(self, rhs:Rhs) -> Option<Self::Output>;
// }


// #[macro_export]
// macro_rules! impl_mul_from_gen_mat_mul {
//     ($name:ident<$f:ident $(,$n:ident)*>) => {
//         impl<$f, Rhs $(,const $n: usize)*> std::ops::Mul<Rhs> for $name<$f $(,$n)*> where Self : GeneralizedMatrixProduct<Rhs> {
//             type Output=<Self as GeneralizedMatrixProduct<Rhs>>::Output;

//             fn mul(self, rhs: Rhs) -> Self::Output {
//                 self.generalized_matrix_product(rhs)
//             }
//         }
//     };
// }