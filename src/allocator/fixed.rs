use crate::*;
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct FixedAllocator<A: Arena<Generation=()>> {
    next_index: A::Index,
}

impl<I: Index + TryFrom<usize>, A: Arena<Index=I,Generation=()>> FixedAllocator<A> {
    pub fn create(&mut self) -> Id<A> {
        let index = self.next_index;
        self.next_index.increment();
        Id { index, gen: () }
    }
}