pub mod affine_basis;
pub use affine_basis::AffineBasis;

pub mod affine_coordinates;
pub use affine_coordinates::{AffineCoordinatesGen, AffineCoordinatesDyn, AffineCoordinates};

pub mod affine_sub_space;
pub use affine_sub_space::{AffineSubSpaceGen, AffineSubSpaceDyn, AffineSubSpace};

pub mod basis;
pub use basis::Basis;

pub mod sub_space;
pub use sub_space::{SubSpace, SubSpaceDyn, SubSpaceGen};


