use crate::*;

pub trait Indexes<A>: Copy {
    fn index(&self) -> usize;
    fn id(&self) -> Id<A>;
}

pub trait TryIndexes<A>: Copy {
    fn index(&self) -> Option<usize>;
    fn id(&self) -> Option<Id<A>>;
}
