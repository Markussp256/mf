// the standard take while takes also the first item that fails
// if we have a peekable we can only take the items that satisfy the condition

// discussion on https://stackoverflow.com/questions/28776630/implementing-a-cautious-take-while-using-peekable

pub trait Peekable : Iterator {
    // required method
    fn peek(&self) -> Option<&Self::Item>;

    // provided method
    fn cautios_take_while(& mut self, f:impl Fn(&Self::Item) -> bool) -> Vec<Self::Item> {
        let mut vs=Vec::new();
        while self.peek().is_some_and(&f) {
            vs.push(self.next().unwrap());
        }
        vs
    }

    // unlike check, test does not iterate if the test fails, so we can try something else
    fn test(& mut self, item:Self::Item) -> bool where Self::Item : Eq {
        if self.peek() == Some(&item) {
            self.next();
            true
        } else {
            false
        }
    }
}

pub struct Peeker<I:Iterator> {
    iter: I,
    next: Option<I::Item>,
}

impl<I:Iterator> Peeker<I> {
    pub fn new(mut iter:I) -> Self {
        let next=iter.next();
        Self { iter, next }
    }
}


impl<I:Iterator> From<I> for Peeker<I> {
    fn from(mut iter:I) -> Self {
        let next=iter.next();
        Self{ iter, next }
    }
}

impl<I:Iterator> Iterator for Peeker<I> {
    type Item=I::Item;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.next.take(); // Move the next to current
        self.next = self.iter.next(); // Load the new next
        next
    }
}

impl<I:Iterator> Peekable for Peeker<I> {
    fn peek(&self) -> Option<&Self::Item> {
        self.next.as_ref()
    }
}


#[test]
fn test_cautious_take_while_peeker() {
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=Peeker::new(iter);
    let res=p.cautios_take_while(|i|i.clone()<2);
    assert_eq!(res,vec![0,1]);
    assert_eq!(p.collect::<Vec<i32>>(),vec![2,3,4]);
}

#[test]
fn test_test_true() {
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=Peeker::new(iter);
    let res=p.test(0);
    assert_eq!(res,true);
    assert_eq!(p.collect::<Vec<i32>>(),vec![1,2,3,4]);
}

#[test]
fn test_test_false() {
    let iter=vec![0,1,2,3,4].into_iter();
    let mut p=Peeker::new(iter);
    let res=p.test(7);
    assert_eq!(res,false);
    assert_eq!(p.collect::<Vec<i32>>(),vec![0,1,2,3,4]);
}