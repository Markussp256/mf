pub mod as_base_matrix;
pub use as_base_matrix::{AsBaseMatrix,AsBaseSquareMatrix};

pub mod from_into_matrix;
pub use from_into_matrix::{FromMatrix,IntoMatrix};

pub mod into_base_matrix;
pub use into_base_matrix::{IntoBaseMatrix, IntoBaseSquareMatrix};

pub mod into_dyn_matrix;
pub use into_dyn_matrix::IntoDynMatrix;

pub mod try_from_into_matrix;
pub use try_from_into_matrix::{TryFromMatrix,TryIntoMatrix};

pub mod try_submatrirx;
pub use try_submatrirx::TrySubMatrix;