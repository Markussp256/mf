
// marker trait if matrix is normal and therefore has orthonormal eigenvectors
// for these matrices we can compute stable eig

use algebra_traits::Conjugate;
use super::MatrixSquare;

pub trait MatrixNormal : MatrixSquare + Conjugate {}