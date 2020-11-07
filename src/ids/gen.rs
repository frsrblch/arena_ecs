use std::num::NonZeroU32;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Gen(NonZeroU32);

impl Default for Gen {
    fn default() -> Self {
        Self::new(1).unwrap()
    }
}

impl Gen {
    pub fn new(gen: u32) -> Option<Self> {
        NonZeroU32::new(gen).map(Self)
    }

    pub fn next(&self) -> Self {
        let gen = self.0.get().wrapping_add(1);
        Self::new(gen).unwrap_or_default()
    }

    pub fn get_bits(&self) -> u64 {
        self.0.get().into()
    }

    pub const SIZE_IN_BITS: u32 = std::mem::size_of::<Gen>() as u32 * 8;

    pub const MASK: u64 =
        0b_00000000_00000000_00000000_00000000_11111111_11111111_11111111_11111111;
}

#[test]
fn mask_test() {
    assert_eq!(2u64.pow(Gen::SIZE_IN_BITS) - 1, Gen::MASK);
}
