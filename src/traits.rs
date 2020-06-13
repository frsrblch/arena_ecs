use crate::*;

pub trait Indexes<A: Arena>: Copy {
    fn index(self) -> usize;
    fn id(self) -> Id<A>;
}

pub trait Validates<'a, A: Arena> {
    type Id: Indexes<A>;
    fn validate(&self, id: Id<A>) -> Option<Self::Id>;
}