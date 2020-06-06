use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Valid<'a, A: Arena> {
    pub id: Id<A>,
    marker: PhantomData<&'a Allocator<A>>,
}

impl<A: Arena> Valid<'_, A> {
    pub(crate) fn new(id: Id<A>) -> Self {
        Valid {
            id,
            marker: PhantomData,
        }
    }
}

impl<A: Arena> Clone for Valid<'_, A> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<A: Arena> Copy for Valid<'_, A> {}

impl<A: Arena> Indexes<A> for Valid<'_, A> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }

    fn id(self) -> Id<A> {
        self.id
    }
}

impl<A: Arena> Indexes<A> for &Valid<'_, A> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }

    fn id(self) -> Id<A> {
        self.id
    }
}

#[derive(Debug)]
pub struct ValidRef<'a, A: Arena> {
    pub id: &'a Id<A>,
}

impl<'a, A: Arena> ValidRef<'a, A> {
    pub(crate) fn new(id: &'a Id<A>) -> Self {
        ValidRef {
            id,
        }
    }
}

impl<A: Arena> Clone for ValidRef<'_, A> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<A: Arena> Copy for ValidRef<'_, A> {}

impl<A: Arena> Indexes<A> for ValidRef<'_, A> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }

    fn id(self) -> Id<A> {
        *self.id
    }
}

impl<A: Arena> Indexes<A> for &ValidRef<'_, A> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }

    fn id(self) -> Id<A> {
        *self.id
    }
}

#[derive(Debug, Copy, Clone, Ord, PartialOrd, Eq, PartialEq, Hash)]
pub struct Valid1<'a, T> {
    id: T,
    marker: PhantomData<&'a ()>,
}

impl<T> Valid1<'_, T> {
    pub fn new(id: T) -> Self {
        Self {
            id,
            marker: PhantomData,
        }
    }
}

impl<A: Arena> Indexes<A> for Valid1<'_, Id<A>> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }
    fn id(self) -> Id<A> {
        self.id
    }    
}

impl<A: Arena> Indexes<A> for Valid1<'_, &Id<A>> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }
    fn id(self) -> Id<A> {
        *self.id
    }    
}

impl<A: Arena> Indexes<A> for &Valid1<'_, Id<A>> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }
    fn id(self) -> Id<A> {
        self.id
    }    
}

impl<A: Arena> Indexes<A> for &Valid1<'_, &Id<A>> {
    fn index(self) -> usize {
        self.id.index.to_usize()
    }
    fn id(self) -> Id<A> {
        *self.id
    }    
}