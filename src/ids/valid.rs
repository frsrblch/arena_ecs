use super::*;
use std::marker::PhantomData;

/// A wrapper that is used show that an Id or collection of Ids are valid for the specified lifetime.
///
/// # Generics
/// 'a - The lifetime that the given wrapper is valid.
#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
pub struct Valid<'a, T> {
    pub value: T,
    marker: PhantomData<&'a T>,
}

impl<'a, T> Valid<'a, T> {
    pub(crate) fn new(value: T) -> Self {
        Self {
            value,
            marker: PhantomData,
        }
    }

    pub fn assert(value: T) -> Self {
        Self::new(value)
    }
}

impl<A> ValidId<A> for Valid<'_, Id<A>> {
    fn index(&self) -> usize {
        self.value.get_index()
    }

    fn id(&self) -> Id<A> {
        self.value
    }
}

impl<'a, A> ValidId<A> for Valid<'_, &'a Id<A>> {
    fn index(&self) -> usize {
        self.value.get_index()
    }

    fn id(&self) -> Id<A> {
        *self.value
    }
}
