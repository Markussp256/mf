use crate::TangentBundle;
use crate::trafos::{log_se::LogSE3, special_euclidean::SE3};

pub type TSE3<T> = TangentBundle<SE3<T>, LogSE3<T>>;
