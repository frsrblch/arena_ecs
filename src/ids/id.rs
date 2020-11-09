use super::*;
use crate::ids::gen::Gen;
use std::cmp::Ordering;
use std::convert::TryFrom;
use std::marker::PhantomData;
use std::num::NonZeroU64;

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct Id<A> {
    // TODO remove old fields for SIMD optimization
    // index: u32,
    // gen: Gen,
    bits: NonZeroU64,
    marker: PhantomData<A>,
}

impl<A> Id<A> {
    pub(crate) fn first(index: u32) -> Self {
        Self::new(index, Gen::default())
    }

    pub(crate) fn new(index: u32, gen: Gen) -> Self {
        let index_bits: u64 = u64::from(index) << Gen::SIZE_IN_BITS;
        let gen_bits: u64 = gen.get_bits();

        let bits: u64 = index_bits + gen_bits;

        // UNWRAP: Gen is based on a non-zero integer, so bits will never be zero
        let bits = NonZeroU64::new(bits).unwrap();

        Self::from_bits(bits)
    }

    pub(crate) fn from_bits(bits: NonZeroU64) -> Self {
        Self {
            bits,
            marker: PhantomData,
        }
    }

    pub(crate) fn index_usize(&self) -> usize {
        let index = self.index_u64();

        usize::try_from(index).unwrap()
    }

    pub(crate) fn index_u32(&self) -> u32 {
        let index = self.index_u64();

        u32::try_from(index).unwrap()
    }

    fn index_u64(&self) -> u64 {
        self.bits.get() >> Gen::SIZE_IN_BITS
    }

    pub(crate) fn gen(&self) -> Gen {
        let gen = self.bits.get() & Gen::MASK;

        u32::try_from(gen).ok().and_then(Gen::new).unwrap()
    }

    pub(crate) fn increment(&mut self) {
        *self = self.next_gen();
    }

    pub(crate) fn next_gen(&self) -> Self {
        Self::new(self.index_u32(), self.gen().next())
    }
}

impl<A> Clone for Id<A> {
    fn clone(&self) -> Self {
        Self::from_bits(self.bits)
    }
}

impl<A> Copy for Id<A> {}

impl<A> PartialEq for Id<A> {
    fn eq(&self, other: &Self) -> bool {
        self.bits.eq(&other.bits)
    }
}

impl<A> Eq for Id<A> {}

impl<A> PartialOrd for Id<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.bits.partial_cmp(&other.bits)
    }
}

impl<A> Ord for Id<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.bits.cmp(&other.bits)
    }
}

impl<A> Hash for Id<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state);
    }
}

impl<A: Arena<Allocator = FixedAllocator<A>>> ValidId<A> for Id<A> {
    fn index(self) -> usize {
        self.index_usize()
    }

    fn id(self) -> Id<A> {
        self
    }
}

impl<A: Arena<Allocator = FixedAllocator<A>>> ValidId<A> for &Id<A> {
    fn index(self) -> usize {
        self.index_usize()
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
