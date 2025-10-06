
use super::MatrixDimensions;
type U2=(usize,usize);


#[derive(Clone, Debug, thiserror::Error, PartialEq)]
#[error("Matrix of size {0} can not be muliplied with matrix of size {1}")]
pub struct MatricesCanNotBeMultipliedError(MatrixDimensions,MatrixDimensions);


impl MatricesCanNotBeMultipliedError {
    pub fn try_new(sz0:&U2, sz1:&U2) -> Result<(),Self> {
        if sz0.1 == sz1.0 {
            Ok(())
        } else {
            Err(Self(sz0.clone().into(),sz1.clone().into()))
        }
    }

    pub fn new(sz0:&U2, sz1:&U2) -> Self {
        Self::try_new(sz0,sz1)
            .unwrap_err()
    }
}