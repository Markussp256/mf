
use container_traits::{IndexOutOfBoundsError, LenNotEqualToRequiredLenError};

use super::MatrixDynamicallySized;
use either::Either;

pub trait MatrixDynamic : MatrixDynamicallySized {

    // required
    fn try_push_row(&mut self, row:Self::Row) -> Result<(),Self::Row>;
    fn try_push_col(&mut self, col:Self::Col) -> Result<(),Self::Col>;

    fn try_pop_row(&mut self) -> Option<Self::Row>;
    fn try_pop_col(&mut self) -> Option<Self::Col>;

    fn try_remove_row(& mut self, index:usize) -> Result<Self::Row, IndexOutOfBoundsError<usize>>;
    fn try_remove_col(& mut self, index:usize) -> Result<Self::Col, IndexOutOfBoundsError<usize>>;

    fn try_insert_row(& mut self, row:Self::Row, index:usize) -> Result<(), Either<LenNotEqualToRequiredLenError, IndexOutOfBoundsError<usize>>>;
    fn try_insert_col(& mut self, col:Self::Col, index:usize) -> Result<(), Either<LenNotEqualToRequiredLenError, IndexOutOfBoundsError<usize>>>;
}