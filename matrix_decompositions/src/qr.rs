pub mod qr_def;
pub use qr_def::{QR, UnitaryQR, OrthogonalQR};

pub mod householder_trafo;
pub use householder_trafo::HouseholderTrafoGeneric;

pub mod qr_impl_base;
pub use qr_impl_base::QRImplBase;

mod qr_impl;