#[macro_use]
extern crate variadic_generics;

pub trait OptionTuple {
    type Values;
    fn tuplewrap(self) -> Option<Self::Values>;
}

va_expand!{ ($va_len:tt) ($($va_idents:ident),+) ($($va_indices:tt),+)
    impl<$($va_idents),+> OptionTuple for ($(Option<$va_idents>,)+) {
        type Values = ($($va_idents,)+);
        fn tuplewrap(self) -> Option<Self::Values> {
            Some((
                $(match self.$va_indices { Some(r) => r, None => return None },)+
            ))
        }
    }
}

fn main() {
    let options = (Some("foo"), Some(42), Some(vec![2, 3, 4]));
    let values = options.clone().tuplewrap().unwrap();
    println!("options: {:?}", options);
    println!("values: {:?}", values);
}

// OUTPUT
//
// options: (Some("foo"), Some(42), Some([2, 3, 4]))
// values: ("foo", 42, [2, 3, 4])

