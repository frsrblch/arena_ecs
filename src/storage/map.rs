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

impl<A: Arena, B: Arena, I: Indexes<A>> Get<I, Id<B>> for IdMap<A, B> {
    fn get(&self, id: I) -> Option<&Id<B>> {
        self.get_id(&id.id())
    }

    fn get_mut(&mut self, id: I) -> Option<&mut Id<B>> {
        self.get_id_mut(&id.id())
    }
}

impl<A: Arena, B: Arena> Insert<Id<A>, Id<B>> for IdMap<A, B> {
    fn insert(&mut self, id: Id<A>, value: Id<B>) {
        self.insert_id(id, value);
    }
}

// impl<A: Arena, B: Arena> Insert<Id<A>, Id<B>> for IdMap<A, B> {
//     fn insert(&mut self, id: Id<A>, value: Id<B>) {
//         self.insert_id(id, value)
//     }
// }

// impl<A: Arena, B: Arena> Insert<&Id<A>, Id<B>> for IdMap<A, B> {
//     fn insert(&mut self, id: &Id<A>, value: Id<B>) {
//         self.insert_id(*id, value)
//     }
// }

// impl<A: Arena, B: Arena> Insert<Valid<'_, A>, Id<B>> for IdMap<A, B> {
//     fn insert(&mut self, id: Valid<A>, value: Id<B>) {
//         self.insert_id(id.id(), value)
//     }
// }

// impl<A: Arena, B: Arena> Insert<&Valid<'_, A>, Id<B>> for IdMap<A, B> {
//     fn insert(&mut self, id: &Valid<A>, value: Id<B>) {
//         self.insert_id(id.id(), value)
//     }
// }

// impl<A: Arena, B: Arena> Insert<ValidRef<'_, A>, Id<B>> for IdMap<A, B> {
//     fn insert(&mut self, id: ValidRef<A>, value: Id<B>) {
//         self.insert_id(id.id(), value)
//     }
// }

// impl<A: Arena, B: Arena> Insert<&ValidRef<'_, A>, Id<B>> for IdMap<A, B> {
//     fn insert(&mut self, id: &ValidRef<A>, value: Id<B>) {
//         self.insert_id(id.id(), value)
//     }
// }

impl<'a, A: Arena, B: Arena> IdMap<A, B> {
    pub fn validate(&'a mut self, va: impl Validates<'a, A>, vb: impl Validates<'a, B>) -> ValidMap<'a, A, B> {
        let to_remove = self.values
            .iter()
            .filter_map(|(a, b)| {
                match (va.validate(*a), vb.validate(*b)) {
                    (Some(_a), Some(_b)) => None,
                    _ => Some(*a),
                }
            })
            .collect::<Vec<Id<A>>>();

        for id in to_remove {
            self.values.remove(&id);
        }

        ValidMap { map: self }
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Id<A>, &Id<B>)> {
        self.values.iter()
    }

    // TODO add remove trait to handle &Id<A> and Id<A>
    pub fn remove(&mut self, id: Id<A>) -> Option<Id<B>> {
        self.values.remove(&id)
    }
}

impl<A: Arena, B: Arena> IdMap<A, B> {
    fn get_id(&self, id: &Id<A>) -> Option<&Id<B>> {
        self.values.get(id)
    }

    fn get_id_mut(&mut self, id: &Id<A>) -> Option<&mut Id<B>> {
        self.values.get_mut(id)
    }

    fn insert_id(&mut self, id: Id<A>, value: Id<B>) {
        self.values.insert(id, value);
    }
}

#[derive(Debug)]
pub struct ValidMap<'a, A: Arena, B: Arena> {
    map: &'a mut IdMap<A, B>,
}

impl<'a, A: Arena, B: Arena> ValidMap<'a, A, B> {
    pub fn get(&'a self, id: &Id<A>) -> Option<ValidRef<'a, B>> {
        self.map.values
            .get(id)
            .map(|id| ValidRef::new(id))
    }

    pub fn iter(&'a self) -> impl Iterator<Item = (ValidRef<'a, A>, ValidRef<'a, B>)> + 'a {
        self.map.values
            .iter()
            .map(|(a, b)| {
                (ValidRef::new(a), ValidRef::new(b))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::{FixedArena, GenerationalArena};

    #[test]
    fn remove_invalid_for_dynamic_a() {
        let mut alloc_a = Allocator::<GenerationalArena>::default();
        let mut alloc_b = Allocator::<FixedArena>::default();
        let mut map = IdMap::<GenerationalArena, FixedArena>::default();

        let to_live = alloc_a.create();
        map.insert(to_live.id(), alloc_b.create());
        let to_live = to_live.id();

        let to_kill = alloc_a.create();
        map.insert(to_kill.id(), alloc_b.create());
        let to_kill = to_kill.id();

        alloc_a.kill(to_kill);

        let map = map.validate(&alloc_a, ());

        // UNCOMMENTING CAUSES COMPILER ERROR, IMMUTABLE BORROW OF ALLOCATORS BY map.validate() PREVENTS MUTABILITY
        // alloc_a.kill(to_live);

        assert_eq!(1, map.map.values.len());
        assert!(map.map.values.contains_key(&to_live));
        assert!(!map.map.values.contains_key(&to_kill));
    }

    #[test]
    fn remove_invalid_for_dynamic_b() {
        let mut alloc_a = Allocator::<FixedArena>::default();
        let mut alloc_b = Allocator::<GenerationalArena>::default();
        let mut map = IdMap::<FixedArena, GenerationalArena>::default();

        let to_live = alloc_b.create().id();
        map.insert(alloc_a.create(), to_live);

        let to_kill = alloc_b.create().id();
        map.insert(alloc_a.create(), to_kill);

        alloc_b.kill(to_kill);

        let map = map.validate((), &alloc_b);

        // UNCOMMENTING CAUSES COMPILER ERROR, IMMUTABLE BORROW OF ALLOCATORS BY map.validate() PREVENTS MUTABILITY
        // alloc_b.kill(to_live);

        assert_eq!(1, map.map.values.len());
        assert!(map.map.values.values().find(|id| to_live.eq(*id)).is_some());
        assert!(map.map.values.values().find(|id| to_kill.eq(*id)).is_none());
    }
}