pub trait FromInner {
    type InnerT;
    fn from_inner(inner:Self::InnerT) -> Self;
}