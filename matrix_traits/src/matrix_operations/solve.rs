use crate::MatrixSolveError;

pub trait TrySolveMatrixSystem<Rhs> {
    type Output;
    fn try_solve_matrix_system(self,rhs:Rhs) -> Result<Self::Output,MatrixSolveError>;
}