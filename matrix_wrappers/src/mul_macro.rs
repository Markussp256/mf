

macro_rules! impl_matrix_matrix_mul {
    (  $( *$rhs_t:ident $(=$res_t:ident)? ),*) => {
        impl<MLhs : matrix_traits::MatrixMatrixMul<Rhs,Output=Res>,
             MRhs : Matrix> matrix_traits::MatrixMatrixMul<$rhs_t> Rhs,Output=Res>,
    }

}

