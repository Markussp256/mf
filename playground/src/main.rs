pub trait ItemT {
    type T;
}

pub trait Iter<T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a;
}

pub trait ContainerView : Iter<Self::T>+ItemT {}

pub trait AnyFromIterator<T,E> : Sized {
    fn any_take_away<I:    Iterator<Item=T>>(oref:Option<&Self>, iter:& mut I) -> Result<Self,E>;
    fn any_from_iter<I:IntoIterator<Item=T>>(oref:Option<&Self>, iter:      I) -> Result<Self,E>;
}

pub trait IntoIter<T> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T>;
}

pub trait Rebind<E> {
    type With<T> : IntoIter<T> + AnyFromIterator<T,E>;
}

trait Container 
  :  ContainerView
    +AnyFromIterator<Self::T,MyError>
    +Rebind<MyError>
    +IntoIter<Self::T> {}

#[derive(Clone,Debug)]
struct MyError;

struct MyArr<T>([T;3]);

impl<T> ItemT for MyArr<T> {
    type T=T;
}

impl<T> Iter<T> for MyArr<T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.0
            .iter()
    }
}

impl<T> ContainerView for MyArr<T> {}


impl<T> AnyFromIterator<T,MyError> for MyArr<T> {
    fn any_from_iter<I:IntoIterator<Item=T>>(oref:Option<&Self>, iter:      I) -> Result<Self,MyError> {
        let mut iter=iter.into_iter();
        Self::any_take_away(oref, & mut iter)
            .and_then(|res|
                match iter.next() {
                    Some(_) => Err(MyError),
                    None => Ok(res)
                })
    }

    fn any_take_away<I:    Iterator<Item=T>>(_:Option<&Self>, iter:& mut I) -> Result<Self,MyError> {
        Ok(MyArr([
            iter.next().ok_or(MyError)?,
            iter.next().ok_or(MyError)?,
            iter.next().ok_or(MyError)?]))
    }
}

impl<T> IntoIter<T> for MyArr<T> {
    fn into_iterator(self) -> impl ExactSizeIterator<Item=T> {
        self.0
            .into_iter()
    }
}

impl<T> Rebind<MyError> for MyArr<T> {
    type With<T2>=MyArr<T2>;
}

impl<T> Container for MyArr<T> {}


struct MyArrView<'a,T>([&'a T;3]);

impl<'a,T> ItemT for MyArrView<'a,T> {
    type T = T;
}

impl<'a,T> Iter<T> for MyArrView<'a,T> {
    fn iter<'b>(&'b self) -> impl ExactSizeIterator<Item=&'b T> where T:'b {
        self.0
            .into_iter()
    }
}

impl<'a,T> ContainerView for MyArrView<'a,T> {}

impl<'a,T> AnyFromIterator<&'a T, MyError> for MyArrView<'a,T> {
    fn any_from_iter<I:IntoIterator<Item=&'a T>>(oref:Option<&Self>, iter:      I) -> Result<Self,MyError> {
        let mut iter=iter.into_iter();
        Self::any_take_away(oref, & mut iter)
            .and_then(|res|
                match iter.next() {
                    Some(_) => Err(MyError),
                    None => Ok(res)
                })
    }

    fn any_take_away<I:    Iterator<Item=&'a T>>(_:Option<&Self>, iter:& mut I) -> Result<Self,MyError> {
        Ok(Self([
            iter.next().ok_or(MyError)?,
            iter.next().ok_or(MyError)?,
            iter.next().ok_or(MyError)?]))
    }  
}


pub trait ContainerViewable : Container {
    type Viewer<'a> where Self : 'a, Self::T : 'a;
    fn as_view<'a>(&'a self) -> Self::Viewer<'a>;
}

impl<T> ContainerViewable for MyArr<T> {
    type Viewer<'a> = MyArrView<'a,T> where T : 'a;
    fn as_view<'a>(&'a self) -> Self::Viewer<'a> {
        <Self::Viewer<'a> as AnyFromIterator<&'a T,MyError>>::any_from_iter(
            None,
            <Self as Iter<T>>::iter(self)
        ).unwrap()
    }
}


fn main() {
}