pub mod chain_exact_size;
pub use chain_exact_size::ChainExactSize;

pub mod current;
pub use current::Current;

pub mod inter_leave;
pub use inter_leave::InterLeave;

pub mod look_ahead;
pub use look_ahead::LookAhead;

pub mod peekable;
pub use peekable::Peekable;

pub mod take_a_skip_b;
pub use take_a_skip_b::TakeASkipB;

pub mod var_step;
pub use var_step::VarStep;

pub mod into_exact_size_iterator;
pub use into_exact_size_iterator::{WithExactSize,IntoExactSizeIterator};

pub mod repeat;
pub use repeat::RepeatN;

pub mod repeater;
pub use repeater::RepeaterN;

// if std::iter::next_chunk get stabilized we can use it instead
pub fn next_chunk<I: Iterator<Item = T>, T, const N: usize>(iter: &mut I) -> Result<[T; N], Vec<T>> {
    // assumes that from_fn queries in the right order, i.e. 0, then 1, then 2, etc. 
    let oarr = std::array::from_fn(|_| iter.next());
    crate::option::unwrap_if_all_are_some_arr(oarr)
        .map_err(|err|err.into_iter()
                         .filter(|o|o.is_some())
                         .map(Option::unwrap)
                         .collect())
}

pub fn next_chunk_dyn<I:Iterator<Item=T>, T>(iter: & mut I, n:usize) -> Result<Vec<T>,Vec<T>> {
    let mut vs=Vec::new();
    for _ in 0..n {
        match iter.next() {
            Some(t) => vs.push(t),
            None => return Err(vs)
        };
    }
    Ok(vs)
}

pub enum AllSameError {
    EmptyIterator,
    NotAllTheSame
}

pub fn all_same<I:IntoIterator<Item = T>, T: PartialEq>(iter: I) -> Result<T,AllSameError> {
    // Get the first item, if any
    let mut iter=iter.into_iter();
    if let Some(first) = iter.next() {
        // Compare all other items to the first
        if iter.all(|x| x == first) {
            Ok(first)
        } else {
            Err(AllSameError::NotAllTheSame)
        }
    } else {
        Err(AllSameError::EmptyIterator)
    }
}

// checks one item, unpacks it if it exists and satisfies f
pub fn check<T:Eq+Clone, I:Iterator<Item=T>>(iter:& mut I, f: impl Fn(&T)-> bool) -> Result<T, Option<T>> {
    let next=iter.next();
    next.clone()
        .filter(f)
        .ok_or(next)
}

pub fn split_iterator<I:Iterator<Item=T>,T>(mut iter: I, n: usize) -> (Vec<T>, I) {
    let collected: Vec<T> = iter.by_ref().take(n).collect();
    (collected, iter)
}