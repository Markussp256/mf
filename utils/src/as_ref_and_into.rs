pub trait AsRefAndInto<T> : AsRef<T>+Into<T> {}

impl<S:AsRef<T>+Into<T>,T> AsRefAndInto<T> for S {}
