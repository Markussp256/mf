

// const RAD2DEG:f32=180.0 / std::f32::consts::PI;

pub mod angle;
pub mod angular_acceleration;
pub mod angular_jerk;
pub mod angular_speed;

pub mod operators;


// reexport
pub mod generic {
    pub use super::{
        angle::Angle_generic::*,
        angular_acceleration::AngularAcceleration_generic::*,
        angular_jerk::AngularJerk_generic::*,
        angular_speed::AngularSpeed_generic::*
    };
}