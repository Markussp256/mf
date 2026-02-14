use crate::{error::MatricesCanNotBeMultipliedError, Matrix, MatrixConstructError, MatrixTryConstruct, MatrixView, RowVector, RowVectorTryConstruct, RowVectorView, ColVectorView, TryVectorMatrixProduct, TryMatrixVectorProduct};


pub trait MatrixMatrixProduct<Rhs : MatrixView=Self> : MatrixView {
    type Output : Matrix;
    fn matrix_matrix_product(&self, rhs:&Rhs) -> Self::Output;
}

pub trait IntoMatrixMatrixProduct<Rhs : MatrixView=Self> : Matrix {
    type Output : Matrix;
    fn into_matrix_matrix_product(self, rhs:&Rhs) -> Self::Output;
}



// impl code using only method from matrix
// we do not implement it directly (provided method) because that would
// put many constraints

pub fn try_matrix_matrix_product_impl
    <'a,
     Lhs    : MatrixView<RowView<'a>=LhsRow>,
     LhsRow : RowVectorView+TryVectorMatrixProduct<Rhs,Output=Out::Row>,
     Rhs    : MatrixView,
     Out    : MatrixTryConstruct>(lhs:&'a Lhs, rhs:&'a Rhs) -> Result<Out,MatrixConstructError> {
        let lhs_dims=lhs.matrix_dimensions();
        let rhs_dims=rhs.matrix_dimensions();
        MatricesCanNotBeMultipliedError::try_new(&lhs_dims,&rhs_dims)?;
        let out=Out::try_from_rows(
                lhs.row_views()
                   .map(|row|row.try_vector_matrix_product(rhs).unwrap()))?;
        let out_dims=out.matrix_dimensions();
        assert_eq!(out_dims.0, lhs_dims.0);
        assert_eq!(out_dims.1, rhs_dims.1);
        Ok(out)
}


// from cols
pub fn try_matrix_matrix_product_impl_from_cols
    <'a,
     Lhs    : MatrixView+TryMatrixVectorProduct<RhsCol,Output=Out::Col>,
     Rhs    : MatrixView<ColView<'a>=RhsCol>,
     RhsCol : ColVectorView,
     Out    : MatrixTryConstruct>(lhs:&'a Lhs, rhs:&'a Rhs) -> Result<Out,MatrixConstructError> {
        let lhs_dims=lhs.matrix_dimensions();
        let rhs_dims=rhs.matrix_dimensions();
        MatricesCanNotBeMultipliedError::try_new(&lhs_dims,&rhs_dims)?;
        let out=Out::try_from_cols(
                rhs.col_views()
                   .map(|col|lhs.try_matrix_vector_product(&col).unwrap()))?;
        let out_dims=out.matrix_dimensions();
        assert_eq!(out_dims.0, lhs_dims.0);
        assert_eq!(out_dims.1, rhs_dims.1);
        Ok(out)
}

// F1     : Mul<F2,Output=F3>,
//      F2,
//      F3     : Zero,
     

pub fn try_into_matrix_matrix_product_impl
    <Lhs    : Clone+Matrix<Row=LhsRow>,
     LhsRow : RowVector+TryVectorMatrixProduct<Rhs,Output=OutRow>,
     Rhs    : MatrixView,
     Out    : MatrixTryConstruct<Row=OutRow>,
     OutRow : RowVectorTryConstruct>(lhs:Lhs, rhs:&Rhs) -> Result<Out,MatrixConstructError> {
        let lhs_dims=lhs.matrix_dimensions();
        let rhs_dims=rhs.matrix_dimensions();
        MatricesCanNotBeMultipliedError::try_new(&lhs_dims,&rhs_dims)?;
        let out=Out::try_from_rows(
                lhs.into_rows()
                    .map(|r|r.try_vector_matrix_product(rhs).unwrap()))?;
        let out_dims=out.matrix_dimensions();
        assert_eq!(out_dims.0, lhs_dims.0);
        assert_eq!(out_dims.1, rhs_dims.1);
        Ok(out)
}


pub trait TryMatrixMatrixProduct<Rhs : MatrixView=Self> : MatrixView {
    type Output : Matrix;
    fn try_matrix_matrix_product(&self, rhs:&Rhs) -> Result<Self::Output,MatrixConstructError>;
}

pub trait TryIntoMatrixMatrixProduct<Rhs : MatrixView=Self> : Matrix {
    type Output : Matrix;
    fn try_into_matrix_matrix_product(self, rhs:&Rhs) -> Result<Self::Output,MatrixConstructError>;
}