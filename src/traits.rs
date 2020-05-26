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

pub trait Create<ROW> {
    type Id;
    fn create(&mut self, row: ROW) -> Self::Id;
}

pub trait CreateLinked<ROW> {
    type Links;
    type Id;
    fn create_linked(&mut self, row: ROW, links: Self::Links) -> Self::Id;
}

pub trait LinkChild<CHILD: Arena>: Arena {
    fn link_child(&mut self, id: Id<Self>, child: Id<CHILD>);
}
