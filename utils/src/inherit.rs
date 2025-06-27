

// #[macro_export]
// macro_rules! inherit_map {
//     ($name:ident<$t:ident $(,$n:ident)*> $($pub:ident)?) => {
//         impl<$t $(, const $n:usize)* > $name<$t $(,$n)*> {
//             $($pub)? fn map<T2>(self, f:impl Fn($t)->  T2) -> $name<T2 $(, $n)*> {
//                 $name(self.0.map(f))
//             }
//         }
//     };
// }