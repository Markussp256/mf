pub mod complex;
pub use complex::{Complex, c64};

// pub mod finite;
// pub use finite::Finite;

pub mod enhanced_array;
pub use enhanced_array::EnhancedArray;

pub mod enhanced_container;
pub use enhanced_container::{EnhancedContainer,IntoEnhancedContainer};

pub mod enhanced_vec;
pub use enhanced_vec::EnhancedVec;

pub mod quaternion;
pub use quaternion::Quaternion;

// UnitVector
pub mod unit_vector;
pub use unit_vector::{Unit, UnitVector, UnitVectorDyn};

// Vector
#[cfg(feature = "nalgebra_support")]
pub mod vector_nalgebra_conv;

pub mod vector;
pub use vector::{VectorDyn, VectorGeneric, Vector, Vector2, Vector3, Vector4};

pub mod polynomial;
pub use polynomial::MultivariatePoly;

pub mod real;

pub mod spline;
pub use spline::Spline;

pub mod utils;
pub use utils::*;
