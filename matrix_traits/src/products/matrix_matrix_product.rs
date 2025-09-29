use std::ops::Mul;
use container_traits::{AnyFromIterator, LinearContainerConstructError};
use num_traits::Zero;

use crate::{TryMatrixVectorProduct, ColVector, ColVectorView, Matrix, MatrixView, MatrixTryConstruct};


pub trait MatrixMatrixProduct<Rhs : MatrixView=Self> {
    type Output : MatrixView;
    fn matrix_matrix_product(&self, rhs:&Rhs) -> Self::Output;
}

pub trait IntoMatrixMatrixProduct<Rhs : Matrix=Self> {
    type Output : MatrixView;
    fn into_matrix_matrix_product(self, rhs:Rhs) -> Self::Output;
}



// impl code using only method from matrix
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_matrix_matrix_product_impl
    <F1     : Mul<F2,Output=F3>,
     F2     : Clone,
     F3     : Zero,
     Lhs    : MatrixView<T=F1>+TryMatrixVectorProduct<RhsCol,Output=Out::Col>,
     Rhs    : MatrixView<T=F2,ColView=RhsCol>,
     Out    : MatrixTryConstruct<T=F3>,
     RhsCol : ColVectorView<T=F2>+AnyFromIterator<F2,LinearContainerConstructError>>(lhs:&Lhs, rhs:&Rhs) -> Option<Out> {
        if lhs.ncols() != rhs.nrows() { return None; }
        let lhs_dims=lhs.matrix_dimensions();
        let rhs_dims=rhs.matrix_dimensions();
        let out=Out::try_from_cols(
                (0..rhs.ncols())
                    .map(|j|rhs.col_view(j).unwrap())
                    .map(|col|lhs.try_matrix_vector_product(&col).unwrap())).unwrap();
        let out_dims=out.matrix_dimensions();
        assert_eq!(out_dims.0, lhs_dims.0);
        assert_eq!(out_dims.1, rhs_dims.1);
        Some(out)
        // match &res {
        //     Ok(r) => {
        //         let out_dims=r.matrix_dimensions();
        //         assert_eq!(out_dims.0, lhs_dims.0);
        //         assert_eq!(out_dims.1, rhs_dims.1);
        //     },
        //     Err(e) => { panic!{"matrix_matrix_product error: {:?}",e}; }
        // };
        // res.ok()
}


pub fn try_into_matrix_matrix_product_impl
    <F1     : Mul<F2,Output=F3>,
     F2,
     F3     : Zero,
     Lhs    : Clone+Matrix<T=F1>+TryMatrixVectorProduct<RhsCol,Output=Out::Col>,
     Rhs    : Matrix<T=F2,Col=RhsCol>,
     Out    : MatrixTryConstruct<T=F3>,
     RhsCol : ColVector<T=F2>>(lhs:Lhs, rhs:Rhs) -> Option<Out> {
        if lhs.ncols() != rhs.nrows() { return None; }
        let lhs_dims=lhs.matrix_dimensions();
        let rhs_dims=rhs.matrix_dimensions();
        let out=Out::try_from_cols(
                rhs.into_cols()
                   .map(|col|lhs.try_matrix_vector_product(&col).unwrap())).unwrap();
        let out_dims=out.matrix_dimensions();
        assert_eq!(out_dims.0, lhs_dims.0);
        assert_eq!(out_dims.1, rhs_dims.1);
        Some(out)
}


pub trait TryMatrixMatrixProduct<Rhs : MatrixView=Self> {
    type Output : MatrixView;
    fn try_matrix_matrix_product(&self, rhs:&Rhs) -> Option<Self::Output>;
}

pub trait TryIntoMatrixMatrixProduct<Rhs : Matrix=Self> {
    type Output : MatrixView;
    fn try_into_matrix_matrix_product(self, rhs:Rhs) -> Option<Self::Output>;
}


// impl is for specific types



        // if self.ncols() != rhs.nrows() {
        //     return None;
        // }
        // let ocols:Vec<Result<M::Col,_>>=
        //     <Rhs as Matrix>::into_cols(rhs)
        //         .map(|col|self.clone().try_matrix_vector_product(col).unwrap())
        //         .map(|col|M::Col::any_from_container(col))
        //         .collect();
        // if ocols.iter().any(Result::is_err) {
        //     return None;
        // }
        // M::try_from_cols(
        //     ocols.into_iter()
        //          .map(|ocol|ocol.ok().unwrap())
        //          .collect()).ok()



// impl<M    : Clone+Matrix<Col=Col>+MatrixVectorProduct<Col2,Output=Col3>,
//      M2   : Matrix<Row=Row2,Col=Col2>,
//      M3   : MatrixTryConstruct<F=F3,Row=Row3,Col=Col3>,
//      Col  : ChangeT<F3,Output=Col3>,
//      Row2 : ChangeT<F3,Output=Row3>,
//      Col2,
//      Row3 : BuildMatrix<Col3,Matrix=M3>,
//      Col3 : ColVector<T=F3>,
//      F3> MatrixMatrixProduct<M2> for M {
//     type Output=M3;
//     fn matrix_matrix_product(self, rhs:M2) -> M3 {
//         assert_eq!(self.ncols(), rhs.nrows());
//         M3::try_from_cols(
//             rhs.into_cols()
//                .map(|c2|self.clone().matrix_vector_product(c2))
//         ).ok().unwrap()
//     }
// }

// impl<M    : Clone+Matrix<Col=Col>+TryMatrixVectorProduct<Col2,Output=Col3>,
//      M2   : Matrix<Row=Row2,Col=Col2>,
//      M3   : MatrixTryConstruct<F=F3,Row=Row3,Col=Col3>,
//      Col  : ChangeT<F3,Output=Col3>,
//      Row2 : ChangeT<F3,Output=Row3>,
//      Col2,
//      Row3 : BuildMatrix<Col3,Matrix=M3>,
//      Col3 : ColVector<T=F3>,
//      F3> TryMatrixMatrixProduct<M2> for M {
//     type Output=M3;
//     fn try_matrix_matrix_product(self, rhs:M2) -> Option<M3> {
//         if self.ncols() != rhs.nrows() {
//             return None;
//         }
//         M3::try_from_cols(
//             rhs.into_cols()
//                .map(|c2|self.clone().try_matrix_vector_product(c2).unwrap())
//         ).ok()
//     }
// }

// pub trait MatrixMatrixProduct<RHS:Matrix=Self> : MatrixTryConstruct+Sized
//     where 
//         // Self::F : Clone+Mul<RHS::F>,
//         //    RHS::F : Clone,
//         //  <Self::F as Mul<RHS::F>>::Output : Zero,
//          Self::Row : VectorVectorProduct<RHS::Col> {
//     type Output:MatrixTryConstruct<F=<Self::F as Mul<RHS::F>>::Output>;
//     fn matrix_matrix_product(self, rhs:RHS) -> <Self as MatrixMatrixProduct<RHS>>::Output  {
//         assert_eq!(self.ncols(),rhs.nrows());
//         let f=|i, j|  Some(<Self::Row as VectorVectorProduct<RHS::Col>>::
//                         vector_vector_product(
//                         self.row(i).unwrap(),
//                         rhs.col(j).unwrap()));
//         <<Self as MatrixMatrixProduct<RHS>>::Output as MatrixTryConstruct>::try_from_dim_and_fn(
//                 self.nrows(),
//                 rhs.ncols(),
//                 f).unwrap()
//     }
// }

// pub trait TryMatrixMatrixProduct<RHS:Matrix=Self> : MatrixTryConstruct+Sized
//     where Self::F : Clone+Mul<RHS::F>,
//            RHS::F : Clone,
//          <Self::F as Mul<RHS::F>>::Output : Zero,
//          Self::Row : TryVectorVectorProduct<RHS::Col> {
//     type Output:MatrixTryConstruct<F=<Self::F as Mul<RHS::F>>::Output>;
//     fn try_matrix_matrix_product(self, rhs:RHS) -> Option<<Self as TryMatrixMatrixProduct<RHS>>::Output>  {
//         if self.ncols() != rhs.nrows() {
//             return None;
//         }
//         let f=|i, j|  <Self::Row as TryVectorVectorProduct<RHS::Col>>::
//                         try_vector_vector_product(
//                         self.row(i).unwrap(),
//                         rhs.col(j).unwrap());
//         <<Self as TryMatrixMatrixProduct<RHS>>::Output as MatrixTryConstruct>::try_from_dim_and_fn(
//                 self.nrows(),
//                 rhs.ncols(),
//                 f)
//     }
// }