use crate::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Edge<A> {
    pub from: Id<A>,
    pub to: Id<A>,
}

impl<A> Edge<A> {
    pub fn new(from: Id<A>, to: Id<A>) -> Self {
        Self { from, to }
    }

    pub fn new_valid<'a, I: Indexes<A> + 'a>(from: I, to: I) -> Valid<'a, Self> {
        Valid::new(Self::new(from.id(), to.id()))
    }

    pub fn contains(&self, id: Id<A>) -> bool {
        id == self.from || id == self.to
    }
}

impl<A> PartialEq for Edge<A> {
    fn eq(&self, other: &Self) -> bool {
        self.from.eq(&other.from) && self.to.eq(&other.to)
    }
}

impl<A> Eq for Edge<A> {}

impl<A> PartialOrd for Edge<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(&other).into()
    }
}

impl<A> Ord for Edge<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.from.cmp(&other.from).then_with(|| self.to.cmp(&other.to))
    }
}

impl<A> Hash for Edge<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.from.hash(state);
        self.to.hash(state);
    }
}

impl<A> Clone for Edge<A> {
    fn clone(&self) -> Self {
        Self {
            from: self.from,
            to: self.to
        }
    }
}

impl<A> Copy for Edge<A> {}

impl<A: Arena<Allocator=DynamicAllocator<A>>> Edge<A> {
    pub fn is_alive(&self, allocator: &Allocator<A>) -> bool {
        allocator.is_alive(self.from) && allocator.is_alive(self.to)
    }

    pub fn validate<'a>(&self, allocator: &'a Allocator<A>) -> Option<(Valid<'a, Id<A>>, Valid<'a, Id<A>>)> {
        let a = allocator.validate(self.from)?;
        let b = allocator.validate(self.to)?;
        Some((a, b))
    }
}

impl<'a, A> Valid<'a, Edge<A>> {
    pub fn from(&self) -> Valid<'a, Id<A>> {
        Valid::new(self.value.from)
    }

    pub fn to(&self) -> Valid<'a, Id<A>> {
        Valid::new(self.value.to)
    }
}