// generally solve Ax=b

pub trait TrySolve<B,E> {
    type Output;
    fn try_solve(self, b:B) -> Result<Self::Output,E>;
}

pub trait Solve<B> {
    type Output;
    fn solve(self, b:B) -> Self::Output;
}

// homogeneous, i.e. b=0

pub trait TrySolveHomogeneous<E> {
    type Output;
    fn try_solve_homogeneous(self) -> Result<Self::Output,E>;
}

pub trait SolveHomogeneous {
    type Output;
    fn solve_homogeneous(self) -> Self::Output;
}