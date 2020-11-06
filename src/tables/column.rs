use crate::*;
use std::marker::PhantomData;
use typed_iter::{Iter, IterMut, TypedIterator};

#[derive(Debug)]
pub struct Column<C, T> {
    values: Vec<T>,
    marker: PhantomData<C>,
}

impl<C, T> Default for Column<C, T> {
    fn default() -> Self {
        Self::with_capacity(0)
    }
}

impl<C, T> Column<C, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            values: Vec::with_capacity(capacity),
            marker: PhantomData,
        }
    }

    pub fn push(&mut self, value: T) -> Index<C> {
        let index = self.len();
        self.values.push(value);
        Index::new(index)
    }

    pub fn swap_remove(&mut self, index: &Index<C>) -> T {
        self.values.swap_remove(index.index())
    }

    pub fn get(&self, index: &Index<C>) -> Option<&T> {
        self.values.get(index.index())
    }

    pub fn get_mut(&mut self, index: &Index<C>) -> Option<&mut T> {
        self.values.get_mut(index.index())
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    pub fn is_empty(&self) -> bool {
        self.values.is_empty()
    }

    pub fn iter(&self) -> Iter<C, T> {
        Iter::new(self.values.iter())
    }

    pub fn iter_mut(&mut self) -> IterMut<C, T> {
        IterMut::new(self.values.iter_mut())
    }

    pub fn indices(&self) -> Indices<C> {
        Indices::new(self.len())
    }
}

impl<'a, C, ID> Valid<'a, &Column<C, Id<ID>>>
where
    ID: Arena<Allocator = DynamicAllocator<ID>>,
{
    pub fn iter(&'a self) -> Valid<'a, Iter<C, Id<ID>>> {
        Valid::new(self.value.iter())
    }
}

#[derive(Debug)]
pub struct Indices<'a, C> {
    range: std::ops::Range<usize>,
    marker: PhantomData<&'a C>,
}

impl<'a, C> Indices<'a, C> {
    pub(crate) fn new(len: usize) -> Self {
        Self {
            range: 0..len,
            marker: PhantomData,
        }
    }
}

impl<'a, C> Iterator for Indices<'a, C> {
    type Item = Index<C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.range.next().map(Index::new)
    }
}

impl<'a, C> TypedIterator for Indices<'a, C> {
    type Context = C;
}
