use crate::*;

pub trait Indexes<A>: Copy {
    fn index(&self) -> usize;
    fn id(&self) -> Id<A>;
}