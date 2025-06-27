use container_traits::ContainerMut;
use super::Matrix;

type U2=(usize,usize);
//  IterMut<Self::T> + GetMut<(usize,usize),Self::T> 
pub trait MatrixMut : ContainerMut<U2> + Matrix {

    // provided
    // fn iter_mut_with_indices(&mut self) -> impl ExactSizeIterator<Item=((usize,usize), & mut Self::T)> where Self::T : 'static {
    //     // note: we can not use self.indices because that would give 2 borrows with 1 exclusive
    //     let (nrows,ncols)=self.matrix_dimensions();
    //     MatrixIndexIterator::new(nrows,ncols)
    //             .zip(self.iter_mut())
    // }

    // not possible because matrix is maybe not saved rowwise
    // fn rows_mut(&mut self) -> impl Iterator<Item=& mut Self::Row>;

    // not possible because matrix is maybe not saved colwise
    // fn cols_mut(&mut self) -> impl Iterator<Item=& mut Self::Col>;

    // provided
    // not possible because matrix is maybe not saved rowwise
    // fn get_row_mut(&mut self,i:usize) -> Option<& mut Self::Row>:

    // not possible because matrix is maybe not saved colwise
    // fn get_col_mut(&mut self,j:usize) -> Option<& mut Self::Col>;
}

impl<M:Matrix+ContainerMut<U2>> MatrixMut for M {}
