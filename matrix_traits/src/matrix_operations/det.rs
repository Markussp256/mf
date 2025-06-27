pub trait Det {
    type DetF;
    fn det(self) -> Self::DetF;
}