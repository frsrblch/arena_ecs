use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Valid<'a, A> {
    pub id: Id<A>,
    marker: PhantomData<&'a A>,
}

impl<A> Valid<'_, A> {
    pub(crate) fn new(id: Id<A>) -> Self {
        Valid {
            id,
            marker: PhantomData,
        }
    }
}

impl<A> Clone for Valid<'_, A> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<A> Copy for Valid<'_, A> {}

impl<A> Indexes<A> for Valid<'_, A> {
    fn index(&self) -> usize {
        self.id.get_index()
    }

    fn id(&self) -> Id<A> {
        self.id
    }
}

impl<A> Indexes<A> for &Valid<'_, A> {
    fn index(&self) -> usize {
        self.id.get_index()
    }

    fn id(&self) -> Id<A> {
        self.id
    }
}

#[derive(Debug)]
pub struct ValidRef<'a, A> {
    pub id: &'a Id<A>,
}

impl<'a, A> ValidRef<'a, A> {
    pub(crate) fn new(id: &'a Id<A>) -> Self {
        ValidRef {
            id,
        }
    }
}

impl<A> Clone for ValidRef<'_, A> {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<A> Copy for ValidRef<'_, A> {}

impl<A> Indexes<A> for ValidRef<'_, A> {
    fn index(&self) -> usize {
        self.id.get_index()
    }

    fn id(&self) -> Id<A> {
        *self.id
    }
}

impl<A> Indexes<A> for &ValidRef<'_, A> {
    fn index(&self) -> usize {
        self.id.get_index()
    }

    fn id(&self) -> Id<A> {
        *self.id
    }
}
