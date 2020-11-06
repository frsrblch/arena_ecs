use super::*;
use crate::ids::gen::Gen;
use std::cmp::Ordering;
use std::marker::PhantomData;

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Id<A> {
    pub(crate) index: u32,
    gen: Gen,
    marker: PhantomData<A>,
}

impl<A> Id<A> {
    pub(crate) fn new(index: u32, gen: Gen) -> Self {
        Self {
            index,
            gen,
            marker: PhantomData,
        }
    }

    pub(crate) fn get_index(&self) -> usize {
        self.index as usize
    }

    pub(crate) fn increment(&mut self) {
        *self = self.next_gen();
    }

    pub(crate) fn next_gen(&self) -> Self {
        Self::new(self.index, self.gen.next())
    }
}

impl<A> Clone for Id<A> {
    fn clone(&self) -> Self {
        Self::new(self.index, self.gen)
    }
}

impl<A> Copy for Id<A> {}

impl<A> PartialEq for Id<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index) && self.gen.eq(&other.gen)
    }
}

impl<A> Eq for Id<A> {}

impl<A> PartialOrd for Id<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<A> Ord for Id<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.index
            .cmp(&other.index)
            .then_with(|| self.gen.cmp(&other.gen))
    }
}

impl<A> Hash for Id<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.gen.hash(state);
    }
}

impl<A> Id<A> {
    pub(crate) fn first(index: u32) -> Self {
        Self::new(index, Gen::default())
    }
}

impl<A: Arena<Allocator = FixedAllocator<A>>> ValidId<A> for Id<A> {
    fn index(self) -> usize {
        self.get_index()
    }

    fn id(self) -> Id<A> {
        self
    }
}

impl<A: Arena<Allocator = FixedAllocator<A>>> ValidId<A> for &Id<A> {
    fn index(self) -> usize {
        self.get_index()
    }

    fn id(self) -> Id<A> {
        *self
    }
}

impl<A: Arena<Allocator = DynamicAllocator<A>>> Id<A> {
    pub fn is_alive(&self, allocator: &Allocator<A>) -> bool {
        allocator.is_alive(*self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::{FixedArena, GenerationalArena};
    use std::mem::size_of;

    #[test]
    fn id_size() {
        let id_size = size_of::<Id<FixedArena>>();
        assert_eq!(id_size, size_of::<Id<GenerationalArena>>());
        assert_eq!(id_size, size_of::<Option<Id<FixedArena>>>());
        assert_eq!(id_size, size_of::<Option<Id<GenerationalArena>>>());
    }
}
