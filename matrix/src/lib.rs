pub mod macros;

pub mod matrices;
pub use matrices::{
    Matrix,
    MatrixDyn,
    Matrix2, Matrix3, Matrix4, 
    DiagonalMatrix,
    DiagonalMatrixDyn};

pub mod row_col;
pub use row_col::{MatrixColDyn,MatrixRowDyn,MatrixCol,MatrixRow};