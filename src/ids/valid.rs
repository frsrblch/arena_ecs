use crate::{Id, ValidId};
use std::marker::PhantomData;
use typed_iter::TypedIterator;

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

impl<'a, T: Copy> Valid<'a, &T> {
    pub fn copied(&self) -> Valid<'a, T> {
        Valid::new(*self.value)
    }
}

impl<A> ValidId<A> for Valid<'_, Id<A>> {
    fn index(self) -> usize {
        self.value.index_usize()
    }

    fn id(self) -> Id<A> {
        self.value
    }
}

impl<'a, A> ValidId<A> for Valid<'_, &'a Id<A>> {
    fn index(self) -> usize {
        self.value.index_usize()
    }

    fn id(self) -> Id<A> {
        *self.value
    }
}

impl<'a, C, ID> IntoIterator for Valid<'a, typed_iter::Iter<'a, C, Id<ID>>> {
    type Item = Valid<'a, &'a Id<ID>>;
    type IntoIter = Iter<'a, Id<ID>>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.value.into_iter(),
        }
    }
}

impl<'a, C, ID> TypedIterator for Valid<'a, typed_iter::Iter<'a, C, Id<ID>>> {
    type Context = C;
}

impl<'a, C, ID> IntoIterator for Valid<'a, typed_iter::Iter<'a, C, Option<Id<ID>>>> {
    type Item = Option<Valid<'a, &'a Id<ID>>>;
    type IntoIter = Iter<'a, Option<Id<ID>>>;

    fn into_iter(self) -> Self::IntoIter {
        Iter {
            iter: self.value.into_iter(),
        }
    }
}

impl<'a, C, ID> TypedIterator for Valid<'a, typed_iter::Iter<'a, C, Option<Id<ID>>>> {
    type Context = C;
}

pub struct Iter<'a, ID> {
    iter: std::slice::Iter<'a, ID>,
}

impl<'a, ID> Iterator for Iter<'a, Id<ID>> {
    type Item = Valid<'a, &'a Id<ID>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(Valid::new)
    }
}

impl<'a, ID> Iterator for Iter<'a, Option<Id<ID>>> {
    type Item = Option<Valid<'a, &'a Id<ID>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| id.as_ref().map(Valid::new))
    }
}
