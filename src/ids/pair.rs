use crate::*;
use std::cmp::Ordering;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
pub struct Pair<A> {
    a: Id<A>,
    b: Id<A>,
}

impl<A> Pair<A> {
    pub fn new(a: Id<A>, b: Id<A>) -> Self {
        let (a, b) = (a.min(b), a.max(b));
        Self { a, b }
    }

    pub fn contains(&self, id: Id<A>) -> bool {
        id == self.a || id == self.b
    }

    pub fn a(&self) -> Id<A> {
        self.a
    }

    pub fn b(&self) -> Id<A> {
        self.b
    }
}

impl<A> PartialEq for Pair<A> {
    fn eq(&self, other: &Self) -> bool {
        self.a.eq(&other.a) && self.b.eq(&other.b)
    }
}

impl<A> Eq for Pair<A> {}

impl<A> PartialOrd for Pair<A> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.cmp(&other).into()
    }
}

impl<A> Ord for Pair<A> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.a.cmp(&other.a).then_with(|| self.b.cmp(&other.b))
    }
}

impl<A> Hash for Pair<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.a.hash(state);
        self.b.hash(state);
    }
}

impl<A> Clone for Pair<A> {
    fn clone(&self) -> Self {
        Self {
            a: self.a,
            b: self.b
        }
    }
}

impl<A> Copy for Pair<A> {}

impl<A: Arena<Allocator=DynamicAllocator<A>>> Pair<A> {
    pub fn validate<'a>(&self, allocator: &'a Allocator<A>) -> Option<(Valid<'a, A>, Valid<'a, A>)> {
        let a = allocator.validate(self.a)?;
        let b = allocator.validate(self.b)?;
        Some((a, b))
    }
}