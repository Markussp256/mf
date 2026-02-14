
// marker trait if matrix A is normal,
// meaning that A^HA=AA^H.
// If this criterion is satisfied,
// matrix has orthonormal basis of eigenvectors
// for these matrices we can compute stable eig
// decomposition

use algebra_traits::Conjugate;
use super::MatrixViewSquare;

pub trait MatrixNormal : MatrixViewSquare + Conjugate {}