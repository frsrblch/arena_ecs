use super::*;
use std::cmp::Ordering;
use std::num::NonZeroU32;
use std::marker::PhantomData;
use crate::ids::gen::Gen;

/// A 4-byte generational index that stores the generation in the first byte, and the index in the last 3 bytes.
/// Stores the generation in the first 8 bits, and the index in the last 24 bits.
/// The generation is a NonZeroU8, so the bits are always a valid NonZeroU32.
#[derive(Debug)]
pub struct Id<A> {
    bits: NonZeroU32,
    marker: PhantomData<A>,
}

impl<A> Id<A> {
    pub(crate) fn new(index: u32, gen: Gen) -> Self {
        debug_assert!(index < TWO_POW_24);
        let bits = index << 8 | gen.as_u32();
        // SAFETY: Gen(NonZeroU8) is used in the first byte, therefore any u32 | Gen will return a non-zero u32
        let bits = unsafe { NonZeroU32::new_unchecked(bits) };
        Self::from_non_zero_u32(bits)
    }

    fn from_non_zero_u32(bits: NonZeroU32) -> Self {
        Self {
            bits,
            marker: PhantomData
        }
    }

    pub(crate) fn get_index(&self) -> usize {
        self.get_u32() as usize
    }

    pub(crate) fn get_u32(&self) -> u32 {
        self.bits.get() >> 8
    }

    pub(crate) fn next_gen(&self) -> Self {
        const GEN_MASK: u32 = 0b_00000000_00000000_00000000_11111111;
        const INDEX_MASK: u32 = !GEN_MASK;

        let gen = self.bits.get() & GEN_MASK;
        let index = self.bits.get() & INDEX_MASK;

        // gen cannot overflow 8 bits (255) and cannot be zero
        let gen = match gen {
            255 => 1,
            gen => gen + 1
        };

        // SAFETY: gen is always greater than zero, so index | gen is always greater than zero
        let bits = unsafe { NonZeroU32::new_unchecked(index | gen) };
        Self::from_non_zero_u32(bits)
    }
}

impl<A> Clone for Id<A> {
    fn clone(&self) -> Self {
        Self::from_non_zero_u32(self.bits)
    }
}

impl<A> Copy for Id<A> {}

impl<A> PartialEq for Id<A> {
    fn eq(&self, other: &Self) -> bool {
        self.bits == other.bits
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
        self.bits.cmp(&other.bits)
    }
}

impl<A> Hash for Id<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.bits.hash(state)
    }
}

impl<A> Id<A> {
    pub(crate) fn first(index: u32) -> Self {
        Self::new(index, Gen::default())
    }
}

const TWO_POW_24: u32 = 16_777_216;

impl<A: Arena<Allocator = FixedAllocator<A>>> Indexes<A> for Id<A> {
    fn index(&self) -> usize {
        self.get_index()
    }

    fn id(&self) -> Id<A> {
        *self
    }
}

impl<A: Arena<Allocator = FixedAllocator<A>>> Indexes<A> for &Id<A> {
    fn index(&self) -> usize {
        self.get_index()
    }

    fn id(&self) -> Id<A> {
        **self
    }
}

impl<A: Arena<Allocator = DynamicAllocator<A>>> TryIndexes<A> for Id<A> {
    fn index(&self) -> Option<usize> {
        Some(self.get_index())
    }

    fn id(&self) -> Option<Id<A>> {
        Some(*self)
    }
}

impl<A: Arena<Allocator = DynamicAllocator<A>>> TryIndexes<A> for &Id<A> {
    fn index(&self) -> Option<usize> {
        Some(self.get_index())
    }

    fn id(&self) -> Option<Id<A>> {
        Some(**self)
    }
}

impl<A> TryIndexes<A> for Option<Id<A>> {
    fn index(&self) -> Option<usize> {
        self.map(|id| id.get_index())
    }

    fn id(&self) -> Option<Id<A>> {
        *self
    }
}

impl<A> TryIndexes<A> for &Option<Id<A>> {
    fn index(&self) -> Option<usize> {
        self.map(|id| id.get_index())
    }

    fn id(&self) -> Option<Id<A>> {
        **self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::{FixedArena, GenerationalArena};
    use std::mem::size_of;

    #[test]
    fn id_size() {
        assert_eq!(4, size_of::<Id<FixedArena>>());
        assert_eq!(4, size_of::<Id<GenerationalArena>>());
        assert_eq!(4, size_of::<Option<Id<FixedArena>>>());
        assert_eq!(4, size_of::<Option<Id<GenerationalArena>>>());
    }

    #[test]
    fn as_wrapping() {
        let n = 257u32;
        assert_eq!(1, n as u8);
    }

    #[derive(Debug, Default)]
    struct TestArena;

    fixed_arena!(TestArena);

    #[test]
    fn bit_layout_test() {
        let index = 0b_00000000_00000000_00000000_00001111_u32;

        let gen = Gen::from_u8(0b_00000010_u8).unwrap();

        let id = Id::<TestArena>::new(index, gen);

        let expected = 0b_00000000_00000000_0001111_00000010_u32;

        assert_eq!(expected, id.bits.get());
    }

    #[test]
    fn get_next_gen() {
        let id = Id::<TestArena>::new(314, Gen::from_u8(159).unwrap());
        let next = Id::new(314, Gen::from_u8(160).unwrap());

        assert_eq!(id.next_gen(), next);
    }

    #[test]
    fn get_next_gen_wraps() {
        let id = Id::<TestArena>::new(314, Gen::from_u8(255).unwrap());
        let next = Id::new(314, Gen::default());

        assert_eq!(id.next_gen(), next);
    }
}