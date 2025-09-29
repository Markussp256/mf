
pub trait TryInvCoarse {
    type Output;
    type Error;
    fn try_inv_coarse(self) -> Result<Self::Output,Self::Error>;
}