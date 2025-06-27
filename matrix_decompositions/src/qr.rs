pub mod qr_def;
pub use qr_def::{QR, UnitaryQR, OrthogonalQR};

pub mod householder_trafo;
pub use householder_trafo::HouseholderTrafoGeneric;

mod qr_impl_base;
use qr_impl_base::qr_impl_base;

mod qr_impl;