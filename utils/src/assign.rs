
// assigns a to b
// returns false if the values were already equal 

pub fn assign<T: PartialEq>(a: &mut T, b: T) -> bool {
    let rv=*a != b;
    if rv { *a =b }
    rv
}