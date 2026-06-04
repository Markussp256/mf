
use generic_array::{ArrayLength, GenericArray};

// if std::iter::next_chunk get stabilized we can use it instead
pub fn next_chunk_gen_arr<I: Iterator<Item = T>, T, N: ArrayLength>(iter: &mut I) -> Result<GenericArray<T,N>, Vec<T>> {
    let opt_iter=std::iter::from_fn(||Some(iter.next()));
    let oarr = GenericArray::<Option<T>,N>::try_from_iter(opt_iter).unwrap();
    crate::option::unwrap_if_all_are_some_gen_arr(oarr)
        .map_err(|err|err.into_iter()
                         .filter_map(|o|o)
                         .collect())
}

pub fn next_chunk<I: Iterator<Item = T>, T, const N:usize>(iter: &mut I) -> Result<[T;N],Vec<T>> {
    // assumes that from_fn queries in the right order, i.e. 0, then 1, then 2, etc.
    let oarr=std::array::from_fn(|_|iter.next());
    crate::option::unwrap_if_all_are_some_arr(oarr)
        .map_err(|err|err.into_iter()
                         .filter_map(|o|o)
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