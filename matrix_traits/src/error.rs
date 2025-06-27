pub mod matrix_can_not_be_multiplied;
pub use matrix_can_not_be_multiplied::{MatrixCanNotBeMultipliedWithVectorError, MatricesCanNotBeMultipliedError};

pub mod matrix_construct_error;
pub use matrix_construct_error::MatrixConstructError;

pub mod matrix_dimensions;
pub use matrix_dimensions::MatrixDimensions;

pub mod matrix_not_regular;
pub use matrix_not_regular::MatrixNotRegularError;

pub mod matrix_solve_error;
pub use matrix_solve_error::MatrixSolveError;