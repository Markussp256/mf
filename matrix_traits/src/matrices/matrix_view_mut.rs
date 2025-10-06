use container_traits::{ContainerViewMut, IndexOutOfBoundsError};
use crate::{ColVectorViewMut, RowVectorViewMut};

use super::MatrixView;

type U2=(usize,usize);

pub trait MatrixViewMut : ContainerViewMut<U2> + MatrixView {
    type RowViewMut<'a> : RowVectorViewMut where Self : 'a;
    type ColViewMut<'a> : ColVectorViewMut where Self : 'a;

    fn try_row_view_mut<'a>(&'a mut self, i:usize) -> Result<Self::RowViewMut<'a>,IndexOutOfBoundsError<usize>>;

    fn row_views_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=Self::RowViewMut<'a>>;

    fn try_col_view_mut<'a>(&'a mut self, j:usize) -> Result<Self::RowViewMut<'a>,IndexOutOfBoundsError<usize>>;

    fn col_views_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=Self::RowViewMut<'a>>;
}
