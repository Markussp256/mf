/// computes the binomial coeffiecient n choose k for k<=6 in compile time
/// k must be provided as a literal, n can be any expression
#[macro_export]
macro_rules! NChooseK {
    ($N:expr,0) => {
        1
    };
    ($N:expr,1) => {
        $N
    };
    ($N:expr,2) => {
        $N * $crate::NChooseK!($N - 1, 1) / 2
    };
    ($N:expr,3) => {
        $N * $crate::NChooseK!($N - 1, 2) / 3
    };
    ($N:expr,4) => {
        $N * $crate::NChooseK!($N - 1, 3) / 4
    };
    ($N:expr,5) => {
        $N * $crate::NChooseK!($N - 1, 4) / 5
    };
    ($N:expr,6) => {
        $N * $crate::NChooseK!($N - 1, 5) / 6
    };
}
