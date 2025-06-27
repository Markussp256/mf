//! Physical Units and Quantities
//! 
//! Each quantity comes with two types, one for absolute measures that only allows conversion to different units
//! and taking difference and one type for differetial measures that allows more mathematical operations (it builds
//! a vectorspace over the corresponding numerical type, which is by default f64).
//! The difference between the two types becomes most obvious with time and duration. 
//! 
//! Units are implemented using traits. For example Millimeters is a trait with methods from_mm to generate a length
//! or position from mm, and method mm to get the length or position in mm.
//! 
//! The types itself are implemented using a numeric value and an enum saying which unit. This results in a larger
//! memory requirement, using f64 it takes 16 bytes to store one item whereas with for example measurements it takes
//! 8 bytes, the same amount as it does to store an f64. The advantage of having the enum is that we do not need to
//! convert to a default unit and therefore can avoid numerical error when constructing and asking for the same unit
//! 
//! Using the macros gen_*_types one can generate types with custom names, e.g.
//!
//! Using the macros
//! unit_trait,
//! unit_conv_consts and
//! quantity
//! one can generate custom quantities with corresponding quantity traits and unit trait(s).

pub mod quantity;

pub mod physical_quantity;
pub use physical_quantity::PhysicalQuantity;

pub mod generic_physical_quantity;
pub use generic_physical_quantity::GenericPhysQuant;

pub mod unit_trait;

pub mod duration;
pub use duration::*;

pub mod angulars;
pub use angulars::{angle::*, angular_acceleration::*, angular_jerk::*, angular_speed::*};

pub mod lengths;
pub use lengths::{acceleration::*, jerk::*, length::*, speed::*, area::*};

crate::unit_trait!(Amperes, amperes, AMPERE);
crate::unit_conv_consts!(AMPERE);
crate::quantity!(positional typename: CurrentMeasure,
                 differential typename: Current,
                 units: Amperes, amperes, AMPERE);

crate::unit_trait!(Newtons, newtons, NEWTON);
crate::unit_conv_consts!(NEWTON);
crate::quantity!(positional typename: ForceMeasure,
                 differential typename: Force,
                 units: Newtons, newtons, NEWTON);

crate::unit_trait!(Hertz, hertz, HERTZ);
crate::unit_conv_consts!(HERTZ);
crate::quantity!(positional typename: FrequencyMeasure,
                 differential typename: Frequency,
                 units: Hertz, hertz, HERTZ);

pub mod generic {
    pub use super::angulars::generic::*;
    pub use super::lengths::generic::*;
    pub use super::Current_generic::*;
    pub use super::Force_generic::*;
    pub use super::Frequency_generic::*;
    pub use super::duration::Duration_generic::*;
}