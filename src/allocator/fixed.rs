use crate::*;
use std::convert::TryFrom;

#[derive(Debug, Default)]
pub struct FixedAllocator<A: Arena<Generation=()>> {
    ids: Vec<Id<A>>,
}

impl<I: Index + TryFrom<usize>, A: Arena<Index=I,Generation=()>> FixedAllocator<A> {
    pub fn create(&mut self) -> Id<A> {
        let index = <A::Index as TryFrom<usize>>::try_from(self.ids.len()).ok().unwrap();
        let id = Id { index, gen: () };
        self.ids.push(id);
        id
    }

    pub fn ids(&self) -> &[Id<A>] {
        &self.ids
    }
}