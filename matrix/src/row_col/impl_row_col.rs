use container_traits::LinearContainer;
use matrix_traits::Transpose;
use super::{MatrixColGeneric, MatrixRowGeneric};

macro_rules! impl_row_col_vector {
    ($name:ident, $lc_name:ident, $other:ident) => {
        paste::paste!(
        impl<C:LinearContainer> Transpose for [<Matrix $name Generic>]<C> {
            type Output=[<Matrix $other Generic>]<C>;
            fn transpose(self) -> Self::Output {
                self.0
                    .into()
            }
        }

        );
    };
}
impl_row_col_vector!(Row,row,Col);
impl_row_col_vector!(Col,col,Row);