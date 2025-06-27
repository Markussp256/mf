use num_traits::Zero;
use std::ops::AddAssign;


pub trait CumSum<T:Zero+AddAssign+Clone> : Sized+IntoIterator<Item=T> {
    fn cum_sum(self) -> impl Iterator<Item=T> {
        self.into_iter()
            .scan(T::zero(), |acc, x| {
                 *acc += x;
                 Some(acc.clone())
             })
    }
}

impl<T:Zero+AddAssign+Clone,S:Sized+IntoIterator<Item=T>> CumSum<T> for S {}


#[test]
fn test_cum_sum() {
    let v=[2,3,4];
    let res:Vec<i32>=v.cum_sum().collect();
    assert_eq!(res,vec![2,5,9]);
}