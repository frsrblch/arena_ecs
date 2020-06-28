use crate::*;

pub trait Indexes<A> {
    fn index(&self) -> usize;
    fn id(&self) -> Id<A>;
}

pub trait Validates<ID, A> {
    fn validate(&self, id: ID) -> Option<Valid<A>>;
}