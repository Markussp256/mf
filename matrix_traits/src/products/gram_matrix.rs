use crate::{matrix::AlgebraMatrix, MatrixConstructError, MatrixTryConstruct};
use container_traits::TryFromFn;
use algebra_traits::{TryScalarproduct, Scalar};

type U2=(usize,usize);

pub trait GramMatrix : AlgebraMatrix where Self::T : Clone+Scalar {
    type Output : MatrixTryConstruct<T=Self::T>;
    fn gram_matrix(self) -> <Self as GramMatrix>::Output where Self::Col : TryScalarproduct<TryScProdT = Self::T> {
        let n=self.ncols();
        <Self::Output as TryFromFn<U2,Self::T,MatrixConstructError>>::try_from_fn((n,n),|(i,j)|self.try_col_sc_prod(i, j).unwrap())
        .ok().unwrap()
    }
}
