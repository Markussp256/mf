
// direct mutable view to a container

#[derive(Debug, container_derive::ContainerMut)]
pub struct ViewMut<'a,C>(&'a mut C);


// impl<'b,T,C:Iter<T>> Iter<T> for ViewMut<'b,C> {
//     fn iter<'a>(&'a self) -> impl ExactSizeIterator<Item=&'a T> where T:'a {
//         self.0
//             .iter()
//     }
// }

// impl<'b,T,C:IterMut<T>> IterMut<T> for ViewMut<'b,C> {
//     fn iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=&'a mut T> where T:'a {
//         self.0
//             .iter_mut()
//     }
// }


// impl<'b,Index,T,C:IndexedIter<Index,T>> IndexedIter<Index,T> for ViewMut<'b,C> {
//     fn indexed_iter<'a>(&'a self) -> impl ExactSizeIterator<Item=(Index,&'a T)> where T : 'a {
//         self.0
//             .indexed_iter()
//     }
// }

// impl<'b,Index,T,C:IndexedIterMut<Index,T>> IndexedIterMut<Index,T> for ViewMut<'b,C> {
//     fn indexed_iter_mut<'a>(&'a mut self) -> impl ExactSizeIterator<Item=(Index,&'a mut T)> where T : 'a {
//         self.0
//             .indexed_iter_mut()
//     }
// }