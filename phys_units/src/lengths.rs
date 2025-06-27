pub mod length;
pub mod area;

pub mod acceleration;
pub mod jerk;
pub mod speed;
pub mod operators;


// reexport
pub mod generic {
    pub use super::{
        length::Length_generic::*,
        area::Area_generic::*,
        acceleration::Acceleration_generic::*,
        jerk::Jerk_generic::*,
        speed::Speed_generic::*
    };
}