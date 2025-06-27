#[macro_export]
macro_rules! one_of_from_name {
    ($name:tt, $($t_name:ident),+) => {

        paste::paste!(
            // Generate the enum
            #[allow(non_camel_case_types)]
            #[derive(Clone, Debug)]
            pub enum $name {
                $($t_name($t_name)),+
            }

            $(
                impl From<$t_name> for $name {
                    fn from(value: $t_name) -> Self {
                        Self::$t_name(value)
                    }
                }


                impl TryInto<$t_name> for $name {
                    type Error = Self;
                    fn try_into(self) -> Result<$t_name, Self::Error> {
                        if let Self::$t_name(value)=self {
                            Ok(value)
                        } else {
                            Err(self)
                        }
                    }
                }

                impl $name {
                    #[allow(non_snake_case)]
                    pub fn $t_name(self) -> Option<$t_name> {
                        if let Self::$t_name(value)=self {
                            Some(value)
                        } else {
                            None
                        }
                    }

                    #[allow(non_snake_case)]
                    pub fn [<is_ $t_name>](&self) -> bool {
                        matches!(self, Self::$t_name(_))
                    }
                }
            )+
        ); // paste
    };
}

#[macro_export]
macro_rules! one_of {
    ($type0:ident, $($type:ident),+) => {
            $crate::one_of_from_name!([<$type0  $(or $type)+>], $type0, $($type),+);
    };
}


#[cfg(test)]
one_of_from_name!(SomeName, f64, i32, bool);

#[cfg(test)]
one_of!(f64, i32, bool);

// Example usages

#[test]
fn test_one_of_f64_case() {
    let a:f64ori32orbool=4.2.into();
    assert!( a.is_f64());
    assert!(!a.is_i32());
    assert!(!a.is_bool());
    assert_eq!(a.clone().f64(),Some(4.2));
    assert_eq!(a.clone().i32(),None);
    assert_eq!(a.clone().bool(),None);
}

#[test]
fn test_one_of_i32_case() {
    let b:f64ori32orbool=3.into();
    assert!(!b.is_f64());
    assert!( b.is_i32());
    assert!(!b.is_bool());
    assert_eq!(b.clone().f64(),None);
    assert_eq!(b.clone().i32(),Some(3));
    assert_eq!(b.clone().bool(),None);
}

#[test]
fn test_one_of_bool_case() {
    let c:f64ori32orbool=true.into();
    assert!(!c.is_f64());
    assert!(!c.is_i32());
    assert!( c.is_bool());
    assert_eq!(c.clone().f64(),None);
    assert_eq!(c.clone().i32(),None);
    assert_eq!(c.clone().bool(),Some(true));
}

