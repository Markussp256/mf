pub mod matrix_can_not_be_multiplied;
pub use matrix_can_not_be_multiplied::MatricesCanNotBeMultipliedError;

pub mod matrix_construct_error;
pub use matrix_construct_error::{MatrixConstructError,MatrixSquareError};

pub mod matrix_dimensions;
pub use matrix_dimensions::MatrixDimensions;

pub mod matrix_regular;
pub use matrix_regular::{MatrixRegularError, MatrixNotRegularError};

pub mod matrix_solve_error;
pub use matrix_solve_error::MatrixSolveError;

pub mod vector_construct_error;
pub use vector_construct_error::{VectorConstructError,MatrixCanNotBeMultipliedWithVectorError, VectorCanNotBeMultipliedWithMatrixError};