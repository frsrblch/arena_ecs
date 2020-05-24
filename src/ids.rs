use crate::*;

#[derive(Debug)]
pub struct Id<A: Arena>
{
    pub(crate) index: A::Index,
    pub(crate) gen: A::Generation,
}

impl<A: Arena> Id<A>
where A::Generation: Fixed
{
    pub(crate) fn index(self) -> usize {
        self.index.index()
    }
}

use std::hash::Hasher;
use std::marker::PhantomData;

impl<A: Arena> Hash for Id<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.gen.hash(state);
    }
}

impl<A: Arena> Clone for Id<A> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            gen: self.gen
        }
    }
}

impl<A: Arena> Copy for Id<A> {}

impl<A: Arena> PartialEq for Id<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index) && self.gen.eq(&other.gen)
    }
}

#[derive(Debug)]
pub struct Valid<'a, A: Arena>
where
    A::Generation: Generation
{
    pub id: Id<A>,
    marker: PhantomData<&'a Allocator<A>>,
}

impl<A: Arena> Valid<'_, A> where A::Generation: Generation {
    pub(crate) fn new(id: Id<A>) -> Self {
        Valid {
            id, marker: PhantomData
        }
    }
}

impl<A: Arena> Clone for Valid<'_, A> where A::Generation: Generation {
    fn clone(&self) -> Self {
        Self::new(self.id)
    }
}

impl<A: Arena> Copy for Valid<'_, A>  where A::Generation: Generation {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Default)]
    struct Body;

    impl Arena for Body {
        type Index = u32;
        type Generation = ();
        type Generations = Self::Index;
        type Dead = ();
    }

    #[test]
    fn id_test() {
        let id = Id::<Body> { index: 0, gen: () };

        assert_eq!(
            id,
            Id {
                index: 0,
                gen: ()
            }
        );
    }
}
