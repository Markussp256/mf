pub mod error;
pub use error::{MatrixConstructError, MatrixNotRegularError, MatrixSolveError};

pub mod extensions;
pub use extensions::{BlockDiagonal,MatrixWithDet};

pub mod matrices;
pub use matrices::*;

pub mod matrix_operations;
pub use matrix_operations::*;

#[cfg(feature = "nalgebra_support")]
mod nalgebra_impl;

pub mod products;
pub use products::*;

pub mod row_col;
pub use row_col::*;