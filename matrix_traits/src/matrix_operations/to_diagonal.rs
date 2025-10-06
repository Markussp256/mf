use crate::{DiagonalMatrixGeneric, Matrix, MatrixSquare};

use container_traits::{AnyFromIterator, ItemT, LinearContainerConstructError};
use num_traits::Zero;

// return matrix where all off_diagonal entries are set to zero

pub fn diagonal_matrix<'a,M:MatrixSquare>(m:&'a M) -> Result<DiagonalMatrixGeneric<M::RowView<'a>>,LinearContainerConstructError>
     where M::T           : Zero+Clone,
           M::RowView<'a> : ItemT<T=M::T>+AnyFromIterator<&'a M::T,LinearContainerConstructError> {
    M::RowView::<'a>::any_from_iter(None, m.diagonal())
        .map(|diag|DiagonalMatrixGeneric::new(diag))
}

pub fn into_diagonal_matrix<M:Matrix+MatrixSquare>(m:M) -> Result<DiagonalMatrixGeneric<M::Row>,LinearContainerConstructError>
     where M::T   : Zero,
           M::Row : ItemT<T=M::T>+AnyFromIterator<M::T,LinearContainerConstructError> {
    M::Row::any_from_iter(None, m.into_diagonal())
        .map(|diag|DiagonalMatrixGeneric::new(diag))
}