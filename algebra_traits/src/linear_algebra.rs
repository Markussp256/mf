pub mod affine_coordinates;
pub use affine_coordinates::{AffineCoordinates, LinearAffineCoordinates};

pub mod affine_basis;
pub use affine_basis::{AffineBasis,TryAffineCombination};

pub mod cross_product;
pub use cross_product::Crossproduct;

pub mod scalar_product;
pub use scalar_product::{Scalarproduct, TryScalarproduct};

pub mod torsor;
pub use torsor::Torsor;

pub mod vectorspace;
pub use vectorspace::*;

pub mod vector;
pub use vector::{ScalarVector, Vector};

pub trait Origin {
    fn origin() -> Self;
}

// pub mod vector_entry;
// pub use vector_entry::VectorEntry;

// pub mod vector_entry_consistent;
// pub use vector_entry_consistent::VectorEntryConsistent;

