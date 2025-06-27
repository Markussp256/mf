use std::iter::{Chain, Take};

use super::{WithExactSize,IntoExactSizeIterator};

pub trait ChainExactSize : Sized+ExactSizeIterator {
    fn chain_exact_size<B: ExactSizeIterator<Item=Self::Item>>(self, rhs:B) -> WithExactSize<Take<Chain<Self,B>>> {
        let len=self.len()+rhs.len();
        self.chain(rhs)
            .into_exact_size_iter(len)
    }
}
impl<A:Sized+ExactSizeIterator> ChainExactSize for A {}


// pub struct ChainExactSize<
//     A:ExactSizeIterator,
//     B:ExactSizeIterator<Item=A::Item>> {
//     a:A,
//     b:B,
//     in_a:bool
// }

// impl<A:ExactSizeIterator,
//      B:ExactSizeIterator<Item=A::Item>> ChainExactSize<A,B> {
//     pub fn new(a:A,b:B) -> Self {
//         Self{a,b,in_a:true}
//     }
// }

// impl<T,
//      A : ExactSizeIterator<Item=T>,
//      B : ExactSizeIterator<Item=T>> Iterator for ChainExactSize<A, B> {
//         type Item=T;
//         fn next(&mut self) -> Option<Self::Item> {  
//             if self.in_a {
//                 let r=self.a.next();
//                 if r.is_some() {
//                     return r;
//                 } else {
//                     self.in_a = false;
//                 }
//             }
//             self.b.next()
//         }
// }

// impl<T,
//      A : ExactSizeIterator<Item=T>,
//      B : ExactSizeIterator<Item=T>> ExactSizeIterator for ChainExactSize<A, B> {
//     fn len(&self) -> usize {
//         if self.in_a {
//             self.a.len()+self.b.len()
//         } else {
//             self.b.len()
//         }
//     }
// }
