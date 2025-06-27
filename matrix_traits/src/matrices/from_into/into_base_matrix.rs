
pub trait IntoBaseMatrix {
    type Output:crate::Matrix;
    fn into_base_matrix(self) -> Self::Output;
}

pub trait IntoBaseSquareMatrix {
    type Output:crate::MatrixSquare;
    fn into_base_square_matrix(self) -> Self::Output;
}

#[macro_export]
macro_rules! into_matrix_from_into_square {
    ($(<$($t:tt)*>)? $type:ty $(where $($wc:tt)*)?) => {
        impl$(<$($t)*>)? $crate::IntoMatrix for $type where Self : $crate::IntoBaseSquareMatrix, $($wc)* {
            type Output=<<Self as $crate::IntoBaseSquareMatrix>::Output as $crate::IntoBaseMatrix>::Output;
            fn into_base_matrix(self) -> Self::Output {
                <<Self as $crate::IntoBaseSquareMatrix>::Output as $crate::IntoBaseMatrix>::into_base_matrix(
                <Self as $crate::IntoBaseSquareMatrix>::into_base_square_matrix(&self))
            }
        }
    };
}