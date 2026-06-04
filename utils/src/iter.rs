pub mod chain_exact_size;
pub use chain_exact_size::ChainExactSize;

pub mod current;
pub use current::Current;

pub mod flatten_exact_size;
pub use flatten_exact_size::FlattenExactSize;

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

pub mod next_chunk;
pub use next_chunk::{next_chunk,next_chunk_gen_arr,next_chunk_dyn};


pub fn try_take_away<T>(iter:& mut impl ExactSizeIterator<Item=T>, n:usize) -> Option<impl ExactSizeIterator<Item=T>> {
    (iter.len() >= n).then(||
    std::iter::from_fn(|| iter.next()).into_exact_size_iter(n))
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