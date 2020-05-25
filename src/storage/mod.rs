pub use component::*;
pub use map::*;

mod component;
mod map;

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
