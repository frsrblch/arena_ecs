use crate::*;

pub trait Fixed: Sized + Debug + Copy + Eq + Hash {
    fn first_gen() -> Self;
}

pub trait Dynamic: Fixed {
    fn next_gen(self) -> Self;
}

macro_rules! dynamic {
($u:ty) => {
    item!{
        pub use std::num:: [<NonZero $u:upper>];

        impl Fixed for [<NonZero $u:upper>] {
            fn first_gen() -> Self {
                [<NonZero $u:upper>]::new(1).unwrap()
            }
        }

        impl Dynamic for [<NonZero $u:upper>] {
            fn next_gen(self) -> Self {
                match self.get() {
                    $u::MAX => Self::first_gen(),
                    gen => Self::new(gen + 1).unwrap()
                }
            }
        }

        #[test]
        fn [<max_ $u _next_is_first>]() {
            assert_eq!([<NonZero $u:upper>]::new($u::MAX).unwrap().next_gen(), [<NonZero $u:upper>]::first_gen());
        }

        #[test]
        fn [<first_ $u _is_followed_by_second>]() {
            assert_eq!([<NonZero $u:upper>]::first_gen().next_gen(), [<NonZero $u:upper>]::new(2).unwrap());
        }
    }
}
}

dynamic!(u8);
dynamic!(u16);
dynamic!(u32);
dynamic!(u64);

impl Fixed for () {
    fn first_gen() -> Self {
        ()
    }
}