pub trait Current : Iterator {
    fn current(&self) -> Option<&Self::Item>;
}

