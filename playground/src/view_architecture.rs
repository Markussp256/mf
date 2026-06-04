trait ItemT { type T; }

pub trait ChangeT<T2> {
    type Output<'a>;
}

trait AnyFromIterator<T> : Sized {
    fn any_from_iter<I:IntoIterator<Item=T>>(iter:I) -> Self;
}

trait Iter<T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a;
}

trait ContainerView : ItemT+Iter<Self::T> {}

trait Container : ContainerView {}

trait ContainerViewable : Container {
    type Viewer<'a> where Self : 'a;
    fn as_view<'a>(&'a self) -> Self::Viewer<'a>;
}

struct MyContainer<T>(T);
impl<T> ItemT          for MyContainer<T> { type T=T; }
impl<T> Iter<T>        for MyContainer<T> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        std::iter::once(&self.0)
    }
}
impl<T,T2> ChangeT<T2> for MyContainer<T> {
    type Output<'a> = MyContainer<T2>;
}
impl<T> ContainerView  for MyContainer<T> {}
impl<T> Container      for MyContainer<T> {}

struct MyContainerView<'a, T>(&'a T);
impl<'a,T> ItemT         for MyContainerView<'a,T> { type T=T; }
impl<'a,T> Iter<T>       for MyContainerView<'a,T> {
    fn iter<'b>(&'b self) -> impl ExactSizeIterator<Item=&'b T>
    where T: 'b {
        std::iter::once(self.0)
    }
}
impl<'a,T,T2:'static> ChangeT<T2> for MyContainerView<'a,T> {
    type Output<'b> = MyContainerView<'b,T2>;
}
impl<'a,T> ContainerView for MyContainerView<'a,T> {}
impl<'a,T> AnyFromIterator<&'a T> for MyContainerView<'a,T> {
    fn any_from_iter<I:IntoIterator<Item=&'a T>>(iter:I) -> Self {
        Self(iter.into_iter().next().unwrap())
    }
}

struct MyWrapper<C>(C);

impl<C : ItemT>         ItemT         for MyWrapper<C> { type T=C::T; }
impl<C : Iter<T>,T>     Iter<T>       for MyWrapper<C> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.0
            .iter()
    }
}
impl<C : AnyFromIterator<T>,T> AnyFromIterator<T> for MyWrapper<C> {
    fn any_from_iter<I:IntoIterator<Item=T>>(iter:I) -> Self {
        Self(C::any_from_iter(iter))
    }
}
impl<C : ContainerView> ContainerView for MyWrapper<C> {}
impl<C : Container>     Container     for MyWrapper<C> {}

struct MyWrapperView<C>(C);

impl<C : ItemT>         ItemT         for MyWrapperView<C> { type T=C::T; }
impl<C : Iter<T>,T>     Iter<T>       for MyWrapperView<C> {
    fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
        self.0
            .iter()
    }
}

impl<C : AnyFromIterator<T>,T> AnyFromIterator<T> for MyWrapperView<C> {
    fn any_from_iter<I:IntoIterator<Item=T>>(iter:I) -> Self {
        Self(C::any_from_iter(iter))
    }
}

impl<C : ContainerView> ContainerView for MyWrapperView<C> {}

impl<T : 'static> ContainerViewable for MyContainer<T> {
    type Viewer<'a> = MyContainerView<'a,T> where Self : 'a;
    fn as_view<'a>(&'a self) -> Self::Viewer<'a> {
        MyContainerView(&self.0)
    }
}

impl<T : 'static,
     C : Container<T=T>> ContainerViewable for MyWrapper<C>
     where for<'a> C : ChangeT<&'a T>,
           for<'a> <C as ChangeT<&'a T>>::Output<'a> : AnyFromIterator<&'a T> {
    type Viewer<'a> = MyWrapperView<<C as ChangeT<&'a T>>::Output<'a>> where Self : 'a;
    fn as_view<'a>(&'a self) -> Self::Viewer<'a> {
        Self::Viewer::<'a>::any_from_iter(self.iter())
    }
}


fn test_is_viewer<'a,T:'static>(v:&'a MyContainer<T>) -> MyContainerView<'a,T> {
    <MyContainer<T> as ContainerViewable>::as_view(v)
}

fn test_is_viewer2<'a,C:ContainerViewable<T=T>,T:'static>(v:&'a MyWrapper<C>) -> MyWrapperView<<C as ChangeT<&'a T>>::Output<'a>>
    where for<'b> C : ChangeT<&'b T>,
          for<'b> <C as ChangeT<&'b T>>::Output<'b> : AnyFromIterator<&'b T> {
    <MyWrapper<C> as ContainerViewable>::as_view(v)
}


// fn test<'a,T>(c:MyContainer<T>) -> impl ContainerViewable<for<b'> Viewer<'b>=MyContainerView<'b,T>> { // <Viewer<'a>=MyContainerView<'a,T>>
//     c
// }


