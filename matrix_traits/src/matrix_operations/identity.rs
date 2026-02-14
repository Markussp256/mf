pub trait Identity {
    fn identity(n:usize) -> Self;
}

pub mod for_static {
    use crate::SquareStaticMatrixView;

    pub trait Identity {
        fn identity() -> Self;
    }

    impl<M:SquareStaticMatrixView+super::Identity> Identity for M {
        fn identity() -> Self {
            <Self as super::Identity>::identity(<Self as SquareStaticMatrixView>::M)
        }
    }

}