
use container_traits::{ContainerView, IndexOutOfBoundsError};
use num_traits::{Zero,One};
use crate::row_col::{RowVectorView,ColVectorView};
use utils::kronecker_delta::kron_delta;
use algebra_traits::{Distance,Tolerance};
type U2=(usize,usize);

// can be dynamic or static sized
pub trait MatrixView : ContainerView<U2> {
    type RowView : RowVectorView;
    type ColView : ColVectorView;
    // provided methods

    fn nrows(&self) -> usize {
        self.size()
            .0
    }

    fn ncols(&self) -> usize {
        self.size()
            .1
    }

    fn matrix_dimensions(&self) -> (usize,usize) {
        (self.nrows(), self.ncols())
    }

    fn len(&self) -> usize {
        self.nrows() * self.ncols()
    }

    fn is_square(&self) -> bool {
        self.nrows() == self.ncols()
    }

    fn is_close_to(&self, rhs:&impl MatrixView<T=Self::T>) -> bool
    where Self::T : Clone+Distance+Tolerance, 
         <Self::T as Distance>::DistT : PartialOrd {
        assert_eq!(self.matrix_dimensions(),rhs.matrix_dimensions());
        self.iter().cloned()
            .zip(rhs.iter().cloned())
            .all(|(l,r)|l.is_close_to(r))
    }

    fn try_row_view(&self, i:usize) -> Result<Self::RowView,IndexOutOfBoundsError<usize>>;

    fn rows(&self) -> impl ExactSizeIterator<Item=Self::RowView> {
        (0..self.nrows())
            .map(|i|self.try_row_view(i).unwrap())
    }

    // not possible because matrix is maybe not saved colwise
    // fn get_col(&self, j:usize) -> Option<&Self::Col>

    fn try_col_view(&self, j:usize) -> Result<Self::ColView,IndexOutOfBoundsError<usize>>;

    fn cols(&self) -> impl ExactSizeIterator<Item=Self::ColView> {
        (0..self.ncols())
            .map(|j|self.try_col_view(j).unwrap())
    }

    fn diagonal(&self) -> impl ExactSizeIterator<Item=&Self::T> {
        let min=Ord::min(self.ncols(),self.nrows());
        (0..min).into_iter()
                .map(|i|self.get((i,i)).unwrap())
    }

    fn is_a_zero_matrix(&self) -> bool where Self::T : Zero {
        self.iter()
            .all(Zero::is_zero)
    }

    fn is_an_identity_matrix(&self) -> bool where Self::T : Zero+One+PartialEq {
        self.is_square()
        && 
        self.indexed_iter()
            .all(|((i,j),aij)| aij == &kron_delta(i,j)) 
    }
}
