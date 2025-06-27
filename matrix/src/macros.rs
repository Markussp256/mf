


#[macro_export]
macro_rules! matrix {
    [$($($aij:expr),*);*] => {
       $crate::matrices::Matrix::from_col_of_rows(
        $crate::row_col::MatrixCol::from(
            [$( $crate::row_col::MatrixRow::from([$( $aij),*])),*]))
    };
}

// macro_rules! as_matrix_into_matrix {
//     ($name:ident) => {
//         impl<M:Matrix> matrix_traits::AsMatrix for $name<M> {
//             type Output=M;
//             fn matrix(&self) -> &M {
//                &self.0
//             }
//         }

//         impl<M:Matrix> matrix_traits::IntoMatrix for $name<M> {
//             type Output=M;
//             fn into_matrix(self) -> M {
//                 self.0
//             }
//         }
//     };
// }
// pub (crate) use as_matrix_into_matrix;


// macro_rules! as_square_into_square {
//         ($name:ident $(, $tr:ident)?) => {
//         impl<M:matrix_traits::MatrixSquare> matrix_traits::AsSquareMatrix for $name<M> $(where M::F : $tr)? {
//             type Output=M;
//             fn square(&self) -> &M {
//                &self.0
//             }
//         }

//         impl<M:matrix_traits::MatrixSquare> matrix_traits::IntoSquareMatrix for $name<M> $(where M::F : $tr)? {
//             type Output=M;
//             fn into_square(self) -> M {
//                 self.0
//             }
//         }

        // impl<M:matrix_traits::MatrixSquare> matrix_traits::AsMatrix for $name<M> $(where M::F : $tr)? {
        //     type Output=<M as matrix_traits::AsMatrix>::Output;
        //     fn matrix(&self) -> &<M as matrix_traits::AsMatrix>::Output {
        //        <M as matrix_traits::AsMatrix>::matrix(&self.0)
        //     }
        // }

        // impl<M:matrix_traits::MatrixSquare> matrix_traits::IntoMatrix for $name<M>$(where M::F : $tr)? {
        //     type Output=<M as matrix_traits::IntoMatrix>::Output;
        //     fn into_matrix(self) -> <M as matrix_traits::IntoMatrix>::Output {
        //         <M as matrix_traits::IntoMatrix>::into_matrix(self.0)
        //     }
        // }
//     };
// }
//  pub (crate) use as_square_into_square;
