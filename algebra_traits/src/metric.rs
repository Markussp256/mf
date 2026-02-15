pub mod metric_torsor;
pub use metric_torsor::MetricTorsor;

pub mod norm;
pub use norm::{Norm, TryNormalize, TryMaxNormOfEntries};

pub mod norm_squared;
pub use norm_squared::NormSquared;

pub mod distance;
pub use distance::{Distance, TryDistance};

pub mod tolerance;
pub use tolerance::Tolerance;