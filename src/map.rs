use crate::*;
use fnv::FnvHashMap;

#[derive(Debug)]
pub struct IdMap<A: Arena, T> {
    values: FnvHashMap<Id<A>, T>,
}

pub trait GetOpt<ID, T> {
    fn get(&self, id: ID) -> Option<&T>;
    fn get_mut(&mut self, id: ID) -> Option<&mut T>;
}

impl<A: Arena<Generation=()>, T> GetOpt<Id<A>, T> for IdMap<A, T> {
    fn get(&self, id: Id<A>) -> Option<&T> {
        self.values.get(&id)
    }

    fn get_mut(&mut self, id: Id<A>) -> Option<&mut T> {
        self.values.get_mut(&id)
    }
}

impl<A: Arena<Generation=()>, T> GetOpt<&Id<A>, T> for IdMap<A, T> {
    fn get(&self, id: &Id<A>) -> Option<&T> {
        self.values.get(id)
    }

    fn get_mut(&mut self, id: &Id<A>) -> Option<&mut T> {
        self.values.get_mut(id)
    }
}

impl<A: Arena<Generation=G>, G: Generation, T> GetOpt<Valid<'_, A>, T> for IdMap<A, T> {
    fn get(&self, id: Valid<A>) -> Option<&T> {
        self.values.get(&id.id)
    }

    fn get_mut(&mut self, id: Valid<A>) -> Option<&mut T> {
        self.values.get_mut(&id.id)
    }
}

impl<A: Arena<Generation=G>, G: Generation, T> GetOpt<&Valid<'_, A>, T> for IdMap<A, T> {
    fn get(&self, id: &Valid<A>) -> Option<&T> {
        self.values.get(&id.id)
    }

    fn get_mut(&mut self, id: &Valid<A>) -> Option<&mut T> {
        self.values.get_mut(&id.id)
    }
}

