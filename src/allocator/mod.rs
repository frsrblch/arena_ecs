use crate::*;
use std::ops::{Deref, DerefMut};
pub use fixed::FixedAllocator;
pub use dynamic::DynamicAllocator;

mod fixed;
mod dynamic;


#[derive(Debug)]
pub struct Allocator<A: Arena> {
    allocator: A::Allocator,
}

impl<A: Arena> Default for Allocator<A> {
    fn default() -> Self {
        Self {
            allocator: A::Allocator::default(),
        }
    }
}

impl<A: Arena> Deref for Allocator<A> {
    type Target = A::Allocator;

    fn deref(&self) -> &Self::Target {
        &self.allocator
    }
}

impl<A: Arena> DerefMut for Allocator<A> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.allocator
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn allocator_size() {
        assert_eq!(24, size_of::<Allocator<FixedArena>>());
        assert_eq!(80, size_of::<Allocator<GenerationalArena>>());
    }

    #[test]
    fn id_size() {
        assert_eq!(1, size_of::<Id<FixedArena>>());
        assert_eq!(2, size_of::<Id<GenerationalArena>>());
    }

    #[derive(Debug, Default)]
    struct FixedArena;

    fixed_arena!(FixedArena, u8);

    #[derive(Debug, Default)]
    struct GenerationalArena;

    dynamic_arena!(GenerationalArena, u8, NonZeroU8);

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
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        assert_eq!(Id { index: 0, gen: NonZeroU8::first() }, gen_allocator.create().id);
        assert_eq!(Id { index: 1, gen: NonZeroU8::first() }, gen_allocator.create().id);
    }

    #[test]
    fn reuse_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        let id1 = gen_allocator.create().id;
        gen_allocator.kill(id1);

        assert_eq!(Id { index: 0, gen: NonZeroU8::first().next_gen() }, gen_allocator.create().id);
        assert_eq!(Id { index: 1, gen: NonZeroU8::first() }, gen_allocator.create().id);
    }

    #[test]
    fn validate_valid_returns_some() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id = allocator.create().id;

        assert!(allocator.validate(id).is_some());
    }

    #[test]
    fn validate_invalid_returns_none() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id = allocator.create().id;
        allocator.kill(id);

        assert!(allocator.validate(id).is_none());
    }

    #[test]
    fn gen_alloc_lifetime_test() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id1 = allocator.create().id; // Remove ".id" to cause a compiler error
        let id2 = allocator.create();

        println!("{:?}", id1);
        println!("{:?}", id2);
    }
}