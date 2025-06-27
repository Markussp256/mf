pub trait Inner {
    type InnerT;
    fn inner(&self) -> &Self::InnerT;
}