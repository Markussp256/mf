use matrix::MatrixDyn;

use super::ProblemBuilderError;

use container_traits::LinearContainerConstructError as LCCE;

#[derive(Debug, thiserror::Error)]
pub enum OptimizationError<F:'static, X, EX=LCCE> {
    #[error("Maximal number of {0} iterations reached")]
    MaximalIteration(u8),

    #[error("Could not construct instance of type X from parameters: {0}")]
    ContainerConstruct(EX),

    #[error("Could not compute distance of {0} to {1}")]
    DifferentOutputDimensions(X, X),

    #[error("Matrix {0} representing the derivative of the provided function at {1} does not have full rank")]
    MatrixNotFullRank(MatrixDyn<F>, X),

    #[error("Problem creating optimization problem {0}")]
    ProblemBuilderError(#[from] ProblemBuilderError)
}