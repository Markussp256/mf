pub mod from_into;
pub use from_into::*;

pub mod matrix_diagonal_type;
pub use matrix_diagonal_type::DiagonalMatrixGeneric;

pub mod matrix_diagonal;
pub use matrix_diagonal::{MatrixDiagonal,MatrixDiagonalTryConstruct};

pub mod matrix_dynamic;
pub use matrix_dynamic::MatrixDynamic;

pub mod matrix_dynamically_sized;
pub use matrix_dynamically_sized::MatrixDynamicallySized;

pub mod matrix_index_iterator;
pub use matrix_index_iterator::MatrixIndexIterator;

pub mod matrix_view_mut;
pub use matrix_view_mut::MatrixViewMut;

pub mod matrix_normal;
pub use matrix_normal::MatrixNormal;

pub mod matrix_shapes;
pub use matrix_shapes::*;

pub mod matrix_try_construct;
pub use matrix_try_construct::MatrixTryConstruct;

pub mod matrix_view;
pub use matrix_view::{MatrixView, AlgebraMatrix};

pub mod matrix_construct;
pub use matrix_construct::MatrixConstruct;

pub mod matrix;
pub use matrix::Matrix;

pub mod static_matrix;
pub use static_matrix::{StaticMatrix,SquareStaticMatrix};