#[macro_export]
macro_rules! from_via {
    (impl $(<$($i1:ident $($i2:ident)? $(: $i3:ident $(< $($i4:ident),* > )?)?),*>)?  From<$a:ty> for $c:ty, via $b:ty $(where $($d:ty : From<$e:ty>),*  )?) => {
        impl $(<$($i1 $($i2)? $(: $i3 $(< $($i4),* > )?)?),*>)? From<$a> for $c $(where $($d : From<$e>),*  )? {
            fn from(a:$a) -> $c {
                let b:$b=a.into();
                b.into()
            }
        }
    }
}

#[macro_export]
macro_rules! into_via {
    (impl $(<$($i1:ident $($i2:ident)? $(: $i3:ident $(< $($i4:ident),* > )?)?),*>)?  Into<$a:ty> for $c:ty, via $b:ty $(where $($d:ty : Into<$e:ty>),*  )?) => {
        impl $(<$($i1 $($i2)? $(: $i3 $(< $($i4),* > )?)?),*>)? Into<$a> for $c $(where $($d : Into<$e>),*  )? {
            fn into(self) -> $a {
                let b:$b=self.into();
                b.into()
            }
        }
    }
}

// ?-operator would require to have implemented From even though we only need Into
// need to convert errror therefore can not just do b.try_into()
// in a previous version we returned either::Either<EC, EB> as an error. However we dont want to have either as a dependency everywhere
#[macro_export]
macro_rules! try_from_via {
    (impl $(<$($i1:ident $($i2:ident)? $(: $i3:ident $(< $($i4:ident),* > )?)?),*>)? TryFrom<$a:ty> for $c:ty , via $b:ty $(where $($d:ty : TryFrom<$e:ty>),*  )? ) => {
        impl $(<$($i1 $($i2)? $(: $i3 $(< $($i4),* > )?)?),*>)? TryFrom<$a> for $c $(where $($d : TryFrom<$e>),*  )? {
            type Error=<$a as TryInto<$b>>::Error;
            fn try_from(a:$a) -> Result<$c, <$a as TryInto<$b>>::Error> {
                let b:$b=a.try_into()?;
                Ok(b.into())
            }
        }
    }
}

#[macro_export]
macro_rules! try_into_via {
    (impl $(<$($i1:ident $($i2:ident)? $(: $i3:ident $(< $($i4:ident),* > )?)?),*>)? TryInto<$a:ty> for $c:ty , via $b:ty $(where $($d:ty : TryInto<$e:ty>),*  )? ) => {
        impl $(<$($i1 $($i2)? $(: $i3 $(< $($i4),* > )?)?),*>)? TryInto<$a> for $c $(where $($d : TryInto<$e>),*  )? {
            type Error=<Self as TryInto<$b>>::Error;
            fn try_into(self) -> Result<$a, <Self as TryInto<$b>>::Error> {
                let b:$b=self.try_into()?;
                Ok(b.into())
            }
        }
    }
}