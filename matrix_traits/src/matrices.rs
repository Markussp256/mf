pub mod from_into;
pub use from_into::*;

pub mod matrix_diagonal;
pub use matrix_diagonal::{MatrixDiagonal,MatrixDiagonalTryConstruct};

pub mod matrix_dynamic;
pub use matrix_dynamic::MatrixDynamic;

pub mod matrix_dynamically_sized;
pub use matrix_dynamically_sized::MatrixDynamicallySized;

pub mod matrix_index_iterator;
pub use matrix_index_iterator::MatrixIndexIterator;

pub mod matrix_mut;
pub use matrix_mut::MatrixMut;

pub mod matrix_normal;
pub use matrix_normal::MatrixNormal;

pub mod matrix_shapes;
pub use matrix_shapes::{MatrixNotTall, MatrixNotWide, MatrixTall, MatrixWide, MatrixSquare, MatrixSquareTryConstruct};

pub mod matrix_try_construct;
pub use matrix_try_construct::MatrixTryConstruct;

pub mod matrix_construct;
pub use matrix_construct::MatrixConstruct;

pub mod matrix;
pub use matrix::{Matrix, AlgebraMatrix};

pub mod static_matrix;
pub use static_matrix::{StaticMatrix,SquareStaticMatrix};