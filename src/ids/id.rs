use super::*;

#[derive(Debug)]
pub struct Id<A: Arena> {
    pub(crate) index: A::Index,
    pub(crate) gen: A::Generation,
}

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
            gen: self.gen,
        }
    }
}

impl<A: Arena> Copy for Id<A> {}

impl<A: Arena> PartialEq for Id<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index) && self.gen.eq(&other.gen)
    }
}

impl<A: Arena> Eq for Id<A> {}

impl<A: Arena<Generation=()>> Indexes<A> for Id<A> {
    fn index(self) -> usize {
        self.index.to_usize()
    }

    fn id(self) -> Id<A> {
        self
    }
}

impl<A: Arena<Generation=()>> Indexes<A> for &Id<A> {
    fn index(self) -> usize {
        self.index.to_usize()
    }

    fn id(self) -> Id<A> {
        *self
    }
}
