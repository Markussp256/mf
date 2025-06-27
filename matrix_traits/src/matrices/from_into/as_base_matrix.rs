
pub trait AsBaseMatrix {
    type Output:crate::Matrix;
    fn base_matrix(&self) -> &Self::Output;
}

pub trait AsBaseSquareMatrix {
    type Output:crate::MatrixSquare;
    fn base_square_matrix(&self) -> &Self::Output;
}

#[macro_export]
macro_rules! as_matrix_from_as_square {
    ($(<$($t:tt)*>)? $type:ty $(where $($wc:tt)*)?) => {
        impl$(<$($t)*>)? $crate::AsMatrix for $type where Self : $crate::AsBaseSquareMatrix, $($wc)* {
            type Output=<<Self as $crate::AsBaseSquareMatrix>::Output as $crate::AsBaseMatrix>::Output;
            fn base_matrix(&self) -> &Self::Output {
                <<Self as $crate::AsBaseSquareMatrix>::Output as $crate::AsMatrix>::base_matrix(
                <Self as $crate::AsBaseSquareMatrix>::base_square_matrix(&self))
            }
        }
    };
}



