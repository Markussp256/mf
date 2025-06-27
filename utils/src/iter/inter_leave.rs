use super::IntoExactSizeIterator;

pub trait InterLeave : Sized+ExactSizeIterator {
    fn inter_leave<S:Clone+ExactSizeIterator<Item=Self::Item>>(self, sep:S) -> impl ExactSizeIterator<Item=Self::Item> {
        let mut iter=self;
        
        // Extract the first item from the main iterator
        let first = iter.next();
        
        let len=iter.len()+(iter.len()-1)*sep.clone().count();

        // Use `flat_map` to interleave elements
        first.into_iter()
             .chain(iter.flat_map(move |item| sep.clone().chain(std::iter::once(item))))
             .into_exact_size_iter(len)
    }
}
impl<I : ExactSizeIterator> InterLeave for I {}