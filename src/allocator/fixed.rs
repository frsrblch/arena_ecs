use crate::*;
use std::marker::PhantomData;

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct FixedAllocator<A> {
    next_index: u32,
    marker: PhantomData<A>,
}

impl<A> FixedAllocator<A> {
    pub fn create(&mut self) -> Id<A> {
        let index = self.next_index;
        self.next_index += 1;
        Id::first(index)
    }

    pub fn ids(&self) -> Ids<A> {
        Ids::new(self)
    }
}

impl<A> Default for FixedAllocator<A> {
    fn default() -> Self {
        Self {
            next_index: 0,
            marker: PhantomData,
        }
    }
}

pub struct Ids<'a, ID> {
    range: std::ops::Range<u32>,
    marker: PhantomData<&'a ID>,
}

impl<'a, ID> Ids<'a, ID> {
    pub fn new(alloc: &'a FixedAllocator<ID>) -> Self {
        Self {
            range: (0..alloc.next_index),
            marker: PhantomData,
        }
    }
}

impl<ID> Iterator for Ids<'_, ID> {
    type Item = Id<ID>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(Id::first)
    }
}

impl<ID> TypedIterator for Ids<'_, ID> {
    type Context = ID;
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::allocator::test::FixedArena;

    #[test]
    fn create_fixed() {
        let mut fixed_allocator = Allocator::<FixedArena>::default();

        assert_eq!(Id::first(0), fixed_allocator.create());
        assert_eq!(Id::first(1), fixed_allocator.create());
    }
}
