use crate::*;

pub trait ValidId<A>: Copy {
    fn index(&self) -> usize;
    fn id(&self) -> Id<A>;
}

pub trait ValidEdge<A> {
    type Id: ValidId<A>;
    fn edge(&self) -> Edge<A>;
    fn from(&self) -> Self::Id;
    fn to(&self) -> Self::Id;
}