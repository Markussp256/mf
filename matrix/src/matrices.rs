
pub mod diag_matrix_generic;
pub use diag_matrix_generic::{DiagonalMatrixDyn, DiagonalMatrixGeneric, DiagonalMatrix};

pub mod matrix_dyn;
pub use matrix_dyn::MatrixDyn;

pub mod matrix_generic;
pub use matrix_generic::MatrixGeneric;

pub mod matrix;
pub use matrix::{Matrix, Matrix2, Matrix3, Matrix4};

pub mod mult_id;
pub use mult_id::MultId;