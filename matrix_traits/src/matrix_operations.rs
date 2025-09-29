pub mod crop_to_square;
pub use crop_to_square::{CropToSquareMatrix,CropToSquareMatrixIfWide,CropToSquareMatrixIfTall,SquareRowSize,SquareColSize};

pub mod det;
pub use det::Det;

pub mod identity;
pub use identity::Identity;

pub mod pop;
pub use pop::{PopRow,PopCol,TryPopRow,TryPopCol};

pub mod push;
pub use push::{PushRow,PushCol,TryPushRow,TryPushCol};

pub mod to_diagonal;
pub use to_diagonal::{diagonal_matrix,into_diagonal_matrix};

pub mod transpose;
pub use transpose::{Transpose, ConjugateTranspose};

pub mod try_inv_coarse;
pub use try_inv_coarse::TryInvCoarse;