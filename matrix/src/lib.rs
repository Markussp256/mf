pub mod macros;

pub mod matrices;
pub use matrices::*;

pub mod row_col;
pub use row_col::{MatrixColDyn,MatrixRowDyn,MatrixCol,MatrixRow,MatrixColView,MatrixRowView};