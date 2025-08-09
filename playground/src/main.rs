use algebra_traits::TryScalarDiv;

#[derive(algebra_derive::TryScalarDiv)]
struct MyArray<T>([T;3]);


fn main() {
}