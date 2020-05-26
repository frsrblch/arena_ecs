use crate::*;
use fnv::FnvHashMap;

#[derive(Debug)]
pub struct IdMap<A: Arena, B: Arena> {
    values: FnvHashMap<Id<A>, Id<B>>,
}

impl<A: Arena, B: Arena> Default for IdMap<A, B> {
    fn default() -> Self {
        Self {
            values: Default::default(),
        }
    }
}

impl<A: Arena<Generation = ()>, B: Arena> GetOpt<Id<A>, Id<B>> for IdMap<A, B> {
    fn get(&self, id: Id<A>) -> Option<&Id<B>> {
        self.values.get(&id)
    }

    fn get_mut(&mut self, id: Id<A>) -> Option<&mut Id<B>> {
        self.values.get_mut(&id)
    }
}

impl<A: Arena<Generation = ()>, B: Arena> GetOpt<&Id<A>, Id<B>> for IdMap<A, B> {
    fn get(&self, id: &Id<A>) -> Option<&Id<B>> {
        self.values.get(id)
    }

    fn get_mut(&mut self, id: &Id<A>) -> Option<&mut Id<B>> {
        self.values.get_mut(id)
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, B: Arena> GetOpt<Valid<'_, A>, Id<B>> for IdMap<A, B> {
    fn get(&self, id: Valid<A>) -> Option<&Id<B>> {
        self.values.get(&id.id)
    }

    fn get_mut(&mut self, id: Valid<A>) -> Option<&mut Id<B>> {
        self.values.get_mut(&id.id)
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, B: Arena> GetOpt<&Valid<'_, A>, Id<B>> for IdMap<A, B> {
    fn get(&self, id: &Valid<A>) -> Option<&Id<B>> {
        self.values.get(&id.id)
    }

    fn get_mut(&mut self, id: &Valid<A>) -> Option<&mut Id<B>> {
        self.values.get_mut(&id.id)
    }
}

impl<A: Arena<Generation = ()>, B: Arena> Insert<Id<A>, Id<B>> for IdMap<A, B> {
    fn insert(&mut self, id: Id<A>, value: Id<B>) {
        self.values.insert(id, value);
    }
}

impl<A: Arena<Generation = ()>, B: Arena> Insert<&Id<A>, Id<B>> for IdMap<A, B> {
    fn insert(&mut self, id: &Id<A>, value: Id<B>) {
        self.values.insert(*id, value);
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, B: Arena> Insert<Valid<'_, A>, Id<B>> for IdMap<A, B> {
    fn insert(&mut self, id: Valid<A>, value: Id<B>) {
        self.values.insert(id.id, value);
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, B: Arena> Insert<&Valid<'_, A>, Id<B>> for IdMap<A, B> {
    fn insert(&mut self, id: &Valid<A>, value: Id<B>) {
        self.values.insert(id.id, value);
    }
}