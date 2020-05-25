use crate::*;
use std::convert::TryFrom;

pub trait Index: Default + Sized + Debug + Copy + Eq + Hash + TryFrom<usize> {
    fn index(self) -> usize;
    fn increment(&mut self);
}

macro_rules! index {
    ($u:ty) => {
        item! {
            impl Index for $u {
                fn index(self) -> usize {
                    self.try_into().unwrap()
                }
                fn increment(&mut self) {
                    *self += 1;
                }
            }

            #[test]
            fn [<$u _conversion_tests>]() {
                assert!(<usize as std::convert::TryFrom<$u>>::try_from($u::MIN).is_ok());
                assert!(<usize as std::convert::TryFrom<$u>>::try_from($u::MAX).is_ok());
            }
        }
    }
}

index!(u8);
index!(u16);
index!(u32);
index!(u64);
index!(usize);
