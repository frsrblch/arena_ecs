use crate::*;

pub trait Indexes<A> {
    fn index(&self) -> usize;
    fn id(&self) -> Id<A>;
}

pub trait TryIndexes<A> {
    fn index(&self) -> Option<usize>;
    fn id(&self) -> Option<Id<A>>;
}
