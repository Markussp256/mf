pub trait IntoInner {
    type InnerT;
    fn into_inner(self) -> Self::InnerT;
}