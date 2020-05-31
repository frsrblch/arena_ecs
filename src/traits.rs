use crate::*;

pub trait Get<ID, T> {
    fn get(&self, id: ID) -> &T;
    fn get_mut(&mut self, id: ID) -> &mut T;
}

pub trait GetOpt<ID, T> {
    fn get(&self, id: ID) -> Option<&T>;
    fn get_mut(&mut self, id: ID) -> Option<&mut T>;
}

pub trait Insert<ID, T> {
    fn insert(&mut self, id: ID, value: T);
}

pub trait Indexes<A: Arena>: Copy {
    fn index(self) -> usize;
    fn id(self) -> Id<A>;
}

pub trait Validates<'a, A: Arena> {
    type Id: Indexes<A>;
    fn validate(&self, id: Id<A>) -> Option<Self::Id>;
}