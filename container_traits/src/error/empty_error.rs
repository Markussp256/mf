#[derive(Clone, Debug, thiserror::Error)]
#[error("provided vec is empty")]
pub struct EmptyVecError;

#[derive(Clone, Debug, thiserror::Error)]
#[error("provided iterator is empty")]
pub struct EmptyIteratorError;
