use crate::*;
use std::convert::{TryFrom, TryInto};

pub trait Index: Debug + Copy + Eq + Hash {
    fn from_usize(index: usize) -> Self;
    fn to_usize(self) -> usize;
    fn increment(&mut self);
}

macro_rules! index {
    ($u:ty) => {
        item! {
            impl Index for $u {
                fn from_usize(index: usize) -> Self {
                    Self::try_from(index).unwrap()
                }

                fn to_usize(self) -> usize {
                    self.try_into().unwrap()
                }

                fn increment(&mut self) {
                    *self += 1;
                }
            }

            #[test]
            fn [<$u _conversion_tests>]() {
                let min = <usize as std::convert::TryFrom<$u>>::try_from($u::MIN);
                let max = <usize as std::convert::TryFrom<$u>>::try_from($u::MAX);

                assert!(min.is_ok());
                assert!(max.is_ok());
            }
        }
    }
}

index!(u8);
index!(u16);
index!(u32);
index!(u64);
index!(usize);
