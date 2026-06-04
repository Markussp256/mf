use container_derive::{IntoIterator,Rebind};

use container_traits::Rebind;

#[derive(IntoIterator, Rebind, Debug, PartialEq)] // 
struct MyWrapper<C>(C);




fn main() {

    let res=<MyWrapper::<[f64;3]> as Rebind<_>>::any_from_iter(None,vec![0, 1, 2]);
    assert_eq!(res, Ok(MyWrapper::<[i32;3]>([0, 1, 2])));

    let mut iter=vec![0, 1, 2, 3].into_iter();
    let res0=MyWrapper::<[f64;3]>::any_take_away(None,& mut iter);
    assert!(res0.is_ok());

    let res1=MyWrapper::<[f64;3]>::any_from_iter(None,vec![0, 1, 2, 3]);
    assert!(res1.is_err());
}