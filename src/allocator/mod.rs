use crate::*;
pub use dynamic::DynamicAllocator;
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

impl<'a, A: Arena, ID> Validates<ID, A> for Allocator<A>
    where
        A::Allocator: Validates<ID, A>,
{
    fn validate(&self, id: ID) -> Option<Valid<A>> {
        self.allocator.validate(id)
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;
    use std::mem::size_of;

    #[derive(Debug, Default)]
    pub(crate) struct FixedArena;

    fixed_arena!(FixedArena);

    #[derive(Debug, Default)]
    pub(crate) struct GenerationalArena;

    dynamic_arena!(GenerationalArena);

    #[test]
    fn allocator_size() {
        assert_eq!(4, size_of::<Allocator<FixedArena>>());
        assert_eq!(80, size_of::<Allocator<GenerationalArena>>());
    }
}
