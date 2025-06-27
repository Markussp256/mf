use algebra_traits::Scalarproduct;
use crate::{ColVector, MatrixConstructError, MatrixTryConstruct};
use container_traits::TryFromFn;

type U2=(usize,usize);

pub trait HermitianOuterProduct : ColVector where Self::T : 'static+Clone+Scalarproduct {
    type Output: MatrixTryConstruct<T= <Self::T as Scalarproduct>::ScProdT>;
    fn hermitian_outer_product(self) -> <Self as HermitianOuterProduct>::Output {
        let n=self.len();
        let v=|i|self.get(i).unwrap().clone();
        let f=|(i,j)|v(i).scalar_product(v(j));
        <<Self as HermitianOuterProduct>::Output as TryFromFn<U2,_,MatrixConstructError>>::try_from_fn(
            (n,n),f).ok().unwrap()
    }
}