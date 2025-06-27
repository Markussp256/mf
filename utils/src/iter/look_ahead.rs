use std::collections::VecDeque;
use super::peekable::Peekable;

// from chat_gpt
// it looks N+1 ahead
pub struct LookAhead<I: Iterator, const N:usize> {
    iter: I,
    buffer: VecDeque<I::Item>
}

impl<I: Iterator, const N:usize> LookAhead<I,N> {
    pub fn new(mut iter: I) -> Self {
        let mut buffer = VecDeque::with_capacity(N + 1);
        for _ in 0..=N {
            if let Some(item) = iter.next() {
                buffer.push_back(item);
            }
        }
        LookAhead {
            iter,
            buffer
        }
    }

    pub fn apply_to_inner<R>(&self,f:impl Fn(&I) -> R ) -> R {
        f(&self.iter)
    }

    pub fn peek_n(&self, n: usize) -> Option<&I::Item> {
        self.buffer.get(n)
    }

    pub fn peek_range(&self, start:usize, end:usize) -> Result<Vec<&I::Item>, Vec<Option<&I::Item>>> {
        crate::option::unwrap_if_all_are_some_vec((start..=end).into_iter().map(|i|self.peek_n(i)).collect())
    }

    pub fn test_multiple<T>(& mut self, test_items:Vec<T>,f:impl Fn(&T,&I::Item)-> bool ) -> bool where I::Item : Eq {
        let n=test_items.len();
        if n == 0 { return true; } // nothing to test
        assert!(n<=N);
        for (i,t) in test_items.iter().enumerate() {
            if !self.peek_n(i).is_some_and(|item|f(t,item)) {
                return false;
            }
        }
        self.nth(n-1);
        true
    }
}

impl<I: Iterator, const N:usize> Iterator for LookAhead<I,N> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        let item=self.buffer.pop_front()?;
        if let Some(next_item) = self.iter.next() {
            self.buffer.push_back(next_item);
        }
        Some(item)
    }
}

impl<I:Iterator, const N:usize> Peekable for LookAhead<I,N> {
    fn peek(&self) -> Option<&Self::Item> {
        self.peek_n(0)
    }
}





#[test]
fn test_cautious_take_while_look_ahead() {
    use std::vec::IntoIter;
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=LookAhead::<IntoIter<i32>,3>::new(iter);
    let res=p.cautios_take_while(|i|i.clone()<2);
    assert_eq!(res,vec![0,1]);
    assert_eq!(p.collect::<Vec<i32>>(),vec![2,3,4]);
}

#[test]
fn test_look_ahead_test_true() {
    use std::vec::IntoIter;
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=LookAhead::<IntoIter<i32>,3>::new(iter);
    let res=p.test(0);
    assert_eq!(res,true);
    assert_eq!(p.collect::<Vec<i32>>(),vec![1,2,3,4]);
}

#[test]
fn test_look_ahead_test_false() {
    use std::vec::IntoIter;
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=LookAhead::<IntoIter<i32>,3>::new(iter);
    let res=p.test(7);
    assert_eq!(res,false);
    assert_eq!(p.collect::<Vec<i32>>(),vec![0,1,2,3,4]);
}

#[test]
fn test_look_ahead_multiple_test_true() {
    use std::vec::IntoIter;
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=LookAhead::<IntoIter<i32>,3>::new(iter);
    let res=p.test_multiple(vec![0,1],|a,b|a==b);
    assert_eq!(res,true);
    assert_eq!(p.collect::<Vec<i32>>(),vec![2,3,4]);
}

#[test]
fn test_look_ahead_multiple_test_false() {
    use std::vec::IntoIter;
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=LookAhead::<IntoIter<i32>,3>::new(iter);
    let res=p.test_multiple(vec![0,7],|a,b|a==b);
    assert_eq!(res,false);
    assert_eq!(p.collect::<Vec<i32>>(),vec![0,1,2,3,4]);
}