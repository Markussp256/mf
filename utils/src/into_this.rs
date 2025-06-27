// unlike into we can provide type as argument for conversion
// if we dont have to provide type we might use into directly

pub trait IntoThis {
    // provided
    fn into_this<T>(self) -> T where Self : Into<T> {
        self.into()
    }
}
// implement for all types
impl<T> IntoThis for T {}

pub trait TryIntoThis {
    // provided 
    fn try_into_this<T>(self) -> Result<T, <Self as TryInto<T>>::Error> where Self:TryInto<T> {
        self.try_into()
    }
}
// implement for all types
impl<T> TryIntoThis for T {}