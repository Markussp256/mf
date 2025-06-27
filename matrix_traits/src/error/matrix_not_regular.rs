use std::fmt::Display;


#[derive(Clone, Debug, thiserror::Error, PartialEq)]
pub struct MatrixNotRegularError;

impl Display for MatrixNotRegularError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "matrix is not regular")
    }
}