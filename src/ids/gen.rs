use std::num::NonZeroU8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Gen(NonZeroU8);

impl Default for Gen {
    fn default() -> Self {
        // SAFETY: 1 != 0
        unsafe {
            Self::from_u8_unchecked(1)
        }
    }
}

impl Gen {
    #[allow(dead_code)]
    pub fn from_u8(gen: u8) -> Option<Self> {
        NonZeroU8::new(gen)
            .map(Gen)
    }

    pub unsafe fn from_u8_unchecked(gen: u8) -> Self {
        Self(NonZeroU8::new_unchecked(gen))
    }

    pub fn as_u32(&self) -> u32 {
        self.0.get() as u32
    }
}
