use crate::*;
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct FixedAllocator<A: Arena<Generation = ()>> {
    next_index: A::Index,
}

impl<I: Index + TryFrom<usize>, A: Arena<Index = I, Generation = ()>> FixedAllocator<A> {
    pub fn create(&mut self) -> Id<A> {
        let index = self.next_index;
        self.next_index.increment();
        Id { index, gen: () }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::allocator::test::FixedArena;

    #[test]
    #[should_panic]
    fn allocator_panic_when_index_out_of_range() {
        let mut allocator = Allocator::<FixedArena>::default();
        for _ in 0..257 {
            let _id = allocator.create();
        }
    }

    #[test]
    fn create_fixed() {
        let mut fixed_allocator = Allocator::<FixedArena>::default();

        assert_eq!(Id { index: 0, gen: () }, fixed_allocator.create());
        assert_eq!(Id { index: 1, gen: () }, fixed_allocator.create());
    }

    #[test]
    fn create_generational() {
        let mut gen_allocator = Allocator::<FixedArena>::default();

        assert_eq!(Id { index: 0, gen: () }, gen_allocator.create());
        assert_eq!(Id { index: 1, gen: () }, gen_allocator.create());
    }
}
