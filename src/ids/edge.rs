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

    pub fn new_valid<'a, I: ValidId<A> + 'a>(from: I, to: I) -> Valid<'a, Self> {
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

    pub fn validate(&self, allocator: &Allocator<A>) -> Option<Valid<Self>> {
        if self.is_alive(allocator) {
            Some(Valid::new(*self))
        } else {
            None
        }
    }
}

impl<A: Arena<Allocator=FixedAllocator<A>>> ValidEdge<A> for Edge<A> {
    type Id = Id<A>;

    fn edge(&self) -> Edge<A> {
        *self
    }

    fn from(&self) -> Id<A> {
        self.from
    }

    fn to(&self) -> Id<A> {
        self.to
    }
}

impl<'a, A: Arena<Allocator=FixedAllocator<A>>> ValidEdge<A> for &'a Edge<A> {
    type Id = Id<A>;

    fn edge(&self) -> Edge<A> {
        **self
    }

    fn from(&self) -> Id<A> {
        self.from
    }

    fn to(&self) -> Id<A> {
        self.to
    }
}

impl<'a, A: Arena<Allocator=DynamicAllocator<A>>> ValidEdge<A> for Valid<'a, Edge<A>> {
    type Id = Valid<'a, Id<A>>;

    fn edge(&self) -> Edge<A> {
        self.value
    }

    fn from(&self) -> Self::Id {
        Valid::new(self.value.from)
    }

    fn to(&self) -> Self::Id {
        Valid::new(self.value.to)
    }
}

impl<'a, 'b, A: Arena<Allocator=DynamicAllocator<A>>> ValidEdge<A> for &'a Valid<'b, Edge<A>> {
    type Id = Valid<'b, Id<A>>;

    fn edge(&self) -> Edge<A> {
        self.value
    }

    fn from(&self) -> Self::Id {
        Valid::new(self.value.from)
    }

    fn to(&self) -> Self::Id {
        Valid::new(self.value.to)
    }
}
