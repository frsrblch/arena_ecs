use crate::*;
pub use alloc_gen::{AllocGen, GenerationCmp};
pub use dynamic::DynamicAllocator;
pub use fixed::FixedAllocator;
use std::ops::{Deref, DerefMut};

mod alloc_gen;
mod dynamic;
mod fixed;

// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Allocator<A: Arena> {
    allocator: A::Allocator,
}

impl<A: Arena<Allocator = ALLOCATOR>, ALLOCATOR: Default> Default for Allocator<A> {
    fn default() -> Self {
        Self {
            allocator: ALLOCATOR::default(),
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

impl<A: Arena<Allocator = DynamicAllocator<A>>> Allocator<A> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            allocator: DynamicAllocator::with_capacity(capacity),
        }
    }
}

#[cfg(test)]
pub(crate) mod test {
    use super::*;

    #[derive(Debug, Default)]
    pub(crate) struct FixedArena;

    fixed_arena!(FixedArena);

    #[derive(Debug, Default)]
    pub(crate) struct GenerationalArena;

    dynamic_arena!(GenerationalArena);
}
