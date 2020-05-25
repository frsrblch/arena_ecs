use super::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Valid<'a, A: Arena>
    where
        A::Generation: Dynamic
{
    pub id: Id<A>,
    marker: PhantomData<&'a Allocator<A>>,
}

impl<A: Arena> Valid<'_, A> where A::Generation: Dynamic {
    pub(crate) fn new(id: Id<A>) -> Self {
        Valid {
            id,
            marker: PhantomData
        }
    }
}

impl<A: Arena> Clone for Valid<'_, A> where A::Generation: Dynamic {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<A: Arena> Copy for Valid<'_, A> where A::Generation: Dynamic {}

