

fn main() {
    let mut foo=Foo(vec![4.0,3.0,1.0]);
    for v in iter_mut(& mut foo) {
        println!("{v}");
    }
}