#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("container is empty")]
pub struct EmptyContainerError;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("provided vec is empty")]
pub struct EmptyVecError;

#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("provided iterator is empty")]
pub struct EmptyIteratorError;