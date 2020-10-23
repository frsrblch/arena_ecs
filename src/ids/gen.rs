use std::num::NonZeroU8;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub(crate) struct Gen(NonZeroU8);

impl Default for Gen {
    fn default() -> Self {
        Self::first()
    }
}

impl Gen {
    pub(crate) fn first() -> Self {
        Self(NonZeroU8::new(1).unwrap())
    }

    pub fn as_u32(&self) -> u32 {
        self.0.get() as u32
    }

    #[cfg(test)]
    pub(crate) fn from_u8(value: u8) -> Option<Self> {
        NonZeroU8::new(value).map(Gen)
    }
}
