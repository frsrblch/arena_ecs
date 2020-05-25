use crate::*;
pub use dynamic::*;
pub use fixed::FixedAllocator;
use std::ops::{Deref, DerefMut};

mod dynamic;
mod fixed;

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
pub(crate) mod test {
    use super::*;
    use std::mem::size_of;

    #[derive(Debug, Default)]
    pub(crate) struct FixedArena;

    fixed_arena!(FixedArena, u8);

    #[derive(Debug, Default)]
    pub(crate) struct GenerationalArena;

    dynamic_arena!(GenerationalArena, u8, NonZeroU8);

    #[test]
    fn allocator_size() {
        assert_eq!(1, size_of::<Allocator<FixedArena>>());
        assert_eq!(80, size_of::<Allocator<GenerationalArena>>());
    }

    #[test]
    fn id_size() {
        assert_eq!(1, size_of::<Id<FixedArena>>());
        assert_eq!(2, size_of::<Id<GenerationalArena>>());
    }
}
