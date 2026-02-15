
use algebra_traits::{Conjugate,RealNumber};


#[derive(algebra_derive::Conjugate)]
struct MyType<C>(C) where C : RealNumber;



fn main() {
}