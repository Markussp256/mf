use crate::{Torsor,IntoDistance,AdditiveGroup};

pub trait MetricTorsor : Torsor
                        +IntoDistance {}

impl<V:AdditiveGroup+IntoDistance> MetricTorsor for V {}