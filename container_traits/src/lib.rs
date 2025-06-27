pub mod array_op;

pub mod container;
pub use container::*;

pub mod error;
pub use error::*;

pub mod linear_container;
pub use linear_container::*;

#[cfg(feature = "nalgebra_support")]
mod nalgebra_impls;

// traits that only make sense for dynamic types
pub mod for_dynamic;
pub use for_dynamic::*;

// traits that are implemented for dynamic and static types
// where for static types there is a more specific trait
pub mod for_dyn_and_stat;
pub use for_dyn_and_stat::*;

pub mod for_static;
pub use for_static::TryFromIterator;

// traits that are implemented for dynamic and static types.
// However there might be more specific trait for dynamic and/or static type 
pub mod for_any;
pub use for_any::*;

pub mod vec_op;


