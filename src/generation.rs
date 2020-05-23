use crate::*;

pub trait Fixed: Sized + Debug + Copy + Eq + Hash {
    type Vec: Clone + Debug + Default + VecType<Item=Self>;
    fn first() -> Self;
}

pub trait Generation: Fixed {
    fn next_gen(self) -> Self;
}

pub trait VecType: Debug + Default + Clone {
    type Item;
    fn pop(&mut self) -> Option<Self::Item>;
    fn push(&mut self, item: Self::Item);
    fn len(&self) -> usize;
    fn get(&self, index: usize) -> Option<&Self::Item>;
    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item>;
}

impl<T: Debug + Clone> VecType for Vec<T> {
    type Item = T;

    fn pop(&mut self) -> Option<Self::Item> {
        self.pop()
    }

    fn push(&mut self, item: Self::Item) {
        Vec::push(self, item);
    }

    fn len(&self) -> usize {
        self.len()
    }

    fn get(&self, index: usize) -> Option<&Self::Item> {
        self.as_slice().get(index)
    }

    fn get_mut(&mut self, index: usize) -> Option<&mut Self::Item> {
        self.as_mut_slice().get_mut(index)
    }
}

macro_rules! generation {
($u:ty) => {
    item!{
        pub use std::num:: [<NonZero $u:upper>];

        impl Fixed for [<NonZero $u:upper>] {
            type Vec = Vec<Self>;
            fn first() -> Self {
                [<NonZero $u:upper>]::new(1).unwrap()
            }
        }

        impl Generation for [<NonZero $u:upper>] {
            fn next_gen(self) -> Self {
                let value = self.get();
                if value == $u::MAX {
                    Self::first()
                } else {
                    Self::new(value + 1).unwrap()
                }
            }
        }

        #[test]
        fn [<max_ $u _next_is_first>]() {
            assert_eq!([<NonZero $u:upper>]::new($u::MAX).unwrap().next_gen(), [<NonZero $u:upper>]::first());
        }

        #[test]
        fn [<first_ $u _is_followed_by_second>]() {
            assert_eq!([<NonZero $u:upper>]::first().next_gen(), [<NonZero $u:upper>]::new(2).unwrap());
        }
    }
}
}

generation!(u8);
generation!(u16);
generation!(u32);
generation!(u64);

impl Fixed for () {
    type Vec = usize;

    fn first() -> Self {
        ()
    }
}

impl VecType for usize {
    type Item = ();

    fn pop(&mut self) -> Option<Self::Item> {
        *self -= 1;
        None
    }

    fn push(&mut self, _item: Self::Item) {
        *self += 1;
    }

    fn len(&self) -> usize {
        *self
    }

    fn get(&self, _index: usize) -> Option<&Self::Item> {
        None
    }

    fn get_mut(&mut self, _index: usize) -> Option<&mut Self::Item> {
        None
    }
}
