pub mod householder_trafo;
pub use householder_trafo::HouseholderTrafoGeneric;

pub mod qr_def;
pub use qr_def::{QRStruct, QRUnitaryStruct, QROrthogonalStruct, QRTrait, QRUnitaryTrait, QROrthogonalTrait};

mod qr_impl_base;
mod qr_impl;