pub trait Identity {
    fn identity(n:usize) -> Self;
}

pub mod for_static {
    use crate::SquareStaticMatrix;

    pub trait Identity {
        fn identity() -> Self;
    }

    impl<M:SquareStaticMatrix+super::Identity> Identity for M {
        fn identity() -> Self {
            <Self as super::Identity>::identity(<Self as SquareStaticMatrix>::M)
        }
    }

}