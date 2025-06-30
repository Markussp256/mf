
use crate::{Matrix, MatrixTryConstruct};


pub trait FixedNumberOfRows : Matrix {
    const NROWS:usize;
}

pub trait FixedNumberOfCols : Matrix {
    const NCOLS:usize;
}


pub trait MatrixNotTall : Matrix {}
pub trait MatrixNotWide : Matrix {}


pub trait MatrixSquare : MatrixNotTall+MatrixNotWide {
    fn n(&self) -> usize {
        let (nrows, ncols)=self.matrix_dimensions();
        assert_eq!(ncols, ncols);
        nrows
    }
}

#[macro_export]
macro_rules! impl_matrix_square {
    ($t:ident $(,$tr:ident)?) => {
        impl<F :'static $(+ $tr)?, const N:usize> $crate::matrices::matrix_shapes::MatrixNotTall   for $t<F,N,N> {}
        impl<F :'static $(+ $tr)?, const N:usize> $crate::matrices::matrix_shapes::MatrixNotWide   for $t<F,N,N> {}
        impl<F :'static $(+ $tr)?, const N:usize> $crate::matrices::matrix_shapes::MatrixSquare    for $t<F,N,N> {}
    };
}

#[macro_export]
macro_rules! impl_matrix_square_one_param {
    ($t:ident $(,$tr:ident)?) => {
        impl<F :'static $(+ $tr)?, const N:usize> $crate::matrices::matrix_shapes::MatrixNotTall for $t<F,N> {}
        impl<F :'static $(+ $tr)?, const N:usize> $crate::matrices::matrix_shapes::MatrixNotWide for $t<F,N> {}
        impl<F :'static $(+ $tr)?, const N:usize> $crate::matrices::matrix_shapes::MatrixSquare  for $t<F,N> {}
    };
}


pub trait MatrixSquareTryConstruct : MatrixSquare + MatrixTryConstruct {}

pub trait MatrixTall : MatrixNotWide {}
pub trait MatrixWide : MatrixNotTall {}

// some fun with macro_rules!
macro_rules! def_stat {
    ($i:literal, $j:literal, $shape:ident) => {
        paste::paste!(
            pub trait [<Matrix $i $j>] : [<Matrix $shape>]+$crate::matrices::static_matrix::StaticMatrix {}
        );
    };
}
#[macro_export]
macro_rules! impl_stat {
    ($t:ident<$f:ident $(: $tr:ident)?, $i:literal,$j:literal>, $shape:ident, $shape_not:ident) => {
        paste::paste!(
            impl<$f :'static $(+ $tr)?> $crate::matrices::matrix_shapes::[<Matrix $shape_not>] for $t<$f,$i,$j> {}
            impl<$f :'static $(+ $tr)?> $crate::matrices::matrix_shapes::[<Matrix $shape>]     for $t<$f,$i,$j> {}
            impl<$f :'static $(+ $tr)?> $crate::matrices::matrix_shapes::[<Matrix $i $j>]      for $t<$f,$i,$j> {}
        );
    };
}

macro_rules! def_square {
    ($($i:literal),*) => {
       $( def_stat!($i,$i,Square); )*
    }
}
def_square!(1,2,3,4,5,6,7,8,9);
#[macro_export]
macro_rules! impl_matrixii {
    ($t:ident , $tr:ident $(,$i:literal)*) => {
        paste::paste!(
        $(impl<F :'static + $tr> $crate::matrices::matrix_shapes::[<Matrix $i $i>] for $t<F,$i,$i> {} )*
        );
    };
    ($t:ident $(,$i:literal)*) => {
        paste::paste!(
        $( impl<F :'static > $crate::matrices::matrix_shapes::[<Matrix $i $i>] for $t<F,$i,$i> {} )*
        );
    }
}
#[macro_export]
macro_rules! impl_matrixii_one_param {
    ($t:ident , $tr:ident $(,$i:literal)*) => {
        paste::paste!(
        $( impl<F :'static +$tr> $crate::matrices::matrix_shapes::[<Matrix $i $i>] for $t<F,$i> {} )*
        );
    };
    ($t:ident $(,$i:literal)*) => {
        paste::paste!(
        $( impl<F :'static  > $crate::matrices::matrix_shapes::[<Matrix $i $i>] for $t<F,$i> {} )*
        );
    }
}
macro_rules! def_tall_or_wide {
    ($i0:literal, $shape:ident) => {};

    ($i0:literal $(,$i:literal)+ ,$shape:ident) => {
        $( def_stat!($i0, $i, $shape); )*
        def_tall_or_wide!($($i),*, $shape);
    }
}
def_tall_or_wide!(1,2,3,4,5,6,7,8,9,Wide);
def_tall_or_wide!(9,8,7,6,5,4,3,2,1,Tall);
#[macro_export]
macro_rules! impl_tall_or_wide {
    ($t:ident $(, $tr:ident)?, $i0:literal, $shape:ident, $shape_not:ident) => {};

    ($t:ident, $i0:literal $(,$i:literal)*, $shape:ident, $shape_not:ident) => {
        $( $crate::impl_stat!($t<F, $i0, $i>, $shape, $shape_not); )*
        $crate::impl_tall_or_wide!($t $(,$i)*,$shape, $shape_not);
    };

    ($t:ident, $tr:ident, $i0:literal $(,$i:literal)*, $shape:ident, $shape_not:ident) => {
        $( $crate::impl_stat!($t<F :$tr, $i0, $i>, $shape, $shape_not); )*
        $crate::impl_tall_or_wide!($t, $tr $(,$i)*,$shape, $shape_not);
    };

}
#[macro_export]
macro_rules! impl_tall_square_and_wide_matrix_marker {
    ($t:ident $(, $tr:ident)?) => {
        $crate::impl_matrix_square!($t $(,$tr)?);
        $crate::impl_matrixii!     ($t $(,$tr)?, 1,2,3,4,5,6,7,8,9);
        $crate::impl_tall_or_wide! ($t $(,$tr)?, 1,2,3,4,5,6,7,8,9,Wide,NotTall);
        $crate::impl_tall_or_wide! ($t $(,$tr)?, 9,8,7,6,5,4,3,2,1,Tall,NotWide);
    };
}