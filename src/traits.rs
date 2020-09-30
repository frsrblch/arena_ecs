use crate::*;

pub trait ValidId<A>: Copy {
    fn index(&self) -> usize;
    fn id(&self) -> Id<A>;
}