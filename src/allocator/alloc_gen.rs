use crate::Id;
use std::cmp::Ordering;
use std::marker::PhantomData;

#[derive(Debug)]
pub enum GenerationCmp<ARENA> {
    Valid,
    OffByOne(Id<ARENA>),
    Outdated,
}

#[derive(Debug)]
pub struct AllocGen<ARENA> {
    gen: u64,
    marker: PhantomData<ARENA>,
}

impl<ARENA> AllocGen<ARENA> {
    const fn new(gen: u64) -> Self {
        Self {
            gen,
            marker: PhantomData,
        }
    }

    pub(crate) fn increment(&mut self) {
        self.gen += 1;
    }

    pub fn min(self, rhs: Self) -> Self {
        Self::new(self.gen.min(rhs.gen))
    }
}

impl<ARENA> std::ops::Sub for AllocGen<ARENA> {
    type Output = u64;

    fn sub(self, rhs: Self) -> Self::Output {
        self.gen - rhs.gen
    }
}

impl<ARENA> Default for AllocGen<ARENA> {
    fn default() -> Self {
        Self::new(0)
    }
}

impl<ARENA> Clone for AllocGen<ARENA> {
    fn clone(&self) -> Self {
        Self::new(self.gen)
    }
}

impl<ARENA> Copy for AllocGen<ARENA> {}

impl<ARENA> PartialEq for AllocGen<ARENA> {
    fn eq(&self, other: &Self) -> bool {
        self.gen.eq(&other.gen)
    }
}

impl<ARENA> Eq for AllocGen<ARENA> {}

impl<ARENA> PartialOrd for AllocGen<ARENA> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.gen.partial_cmp(&other.gen)
    }
}

impl<ARENA> Ord for AllocGen<ARENA> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.gen.cmp(&other.gen)
    }
}
