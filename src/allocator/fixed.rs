use crate::*;
use std::marker::PhantomData;

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
}

impl<A> Default for FixedAllocator<A> {
    fn default() -> Self {
        Self {
            next_index: 0,
            marker: PhantomData,
        }
    }
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
