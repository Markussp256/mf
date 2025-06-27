use crate::{Torsor,Distance,AdditiveGroup};

pub trait MetricTorsor : Torsor
                        +Distance {}

impl<V:AdditiveGroup+Distance> MetricTorsor for V {}