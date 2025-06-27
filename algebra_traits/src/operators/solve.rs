// generally solve Ax=b

pub trait AnySolve<B,E> {
    type Output;
    fn any_solve(self, b:B) -> Result<Self::Output,E>;
}

pub trait TrySolve<B,E> {
    type Output;
    fn try_solve(self, b:B) -> Result<Self::Output,E>;
}

pub trait Solve<B> {
    type Output;
    fn solve(self, b:B) -> Self::Output;
}