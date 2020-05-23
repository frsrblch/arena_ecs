use crate::*;
use std::convert::TryFrom;

pub trait Index: Sized + Debug + Copy + Eq + Hash + Add + TryFrom<usize> {
    type Vec: VecType<Item=Self>;
    fn index(self) -> usize;
}

macro_rules! index {
    ($u:ty) => {
        item! {
            impl Index for $u {
                type Vec = Vec<Self>;
                fn index(self) -> usize {
                    self.try_into().unwrap()
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
