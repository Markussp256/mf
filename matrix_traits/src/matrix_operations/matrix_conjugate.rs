

// impl<M   : MatrixTryConstruct<F=F,Row=Row>+ChangeT<F,Output=Self>,
//      Row : RowVectorTryConstruct<T=F>+ChangeT<F,Output=Row>,
//      F   : Conjugate> GeneralizedMatrixConjugate for M {
//     fn generalized_matrix_conjugate(self) -> Self {
//         self.map(|v|v.conjugate())
//     }
// }