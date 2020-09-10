use crate::*;
use fnv::{FnvHashMap as HashMap};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct IdMap<ID, T> {
    map: HashMap<Id<ID>, T>,
}

impl<ID, T> Default for IdMap<ID, T> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
        }
    }
}

impl<ID, T> IdMap<ID, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, Default::default())
        }
    }

    pub fn insert(&mut self, id: Id<ID>, value: T) {
        self.map.insert(id, value);
    }

    pub fn get(&self, id: impl TryIndexes<ID>) -> Option<&T> {
        let id = id.id()?;
        self.get_unchecked(id)
    }

    pub fn get_mut(&mut self, id: impl TryIndexes<ID>) -> Option<&mut T> {
        let id = id.id()?;
        self.get_unchecked_mut(id)
    }

    fn get_unchecked(&self, id: Id<ID>) -> Option<&T> {
        self.map.get(&id)
    }

    fn get_unchecked_mut(&mut self, id: Id<ID>) -> Option<&mut T> {
        self.map.get_mut(&id)
    }

    pub fn remove(&mut self, id: impl TryIndexes<ID>) -> Option<T> {
        let id = id.id()?;
        self.map.remove(&id)
    }

    pub fn retain<F: FnMut(&Id<ID>, &mut T) -> bool>(&mut self, f: F) {
        self.map.retain(f)
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }
}

impl<ID: Arena<Allocator = DynamicAllocator<ID>>, T> IdMap<ID, T> {
    pub fn retain_living(&mut self, alloc: &Allocator<ID>) {
        self.map.retain(|id, _| alloc.is_alive(*id))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    struct Test;
    dynamic_arena!(Test);

    #[test]
    fn retain_valid() {
        let mut alloc = Allocator::<Test>::default();
        let mut map = IdMap::<Test, u32>::default();

        let a = alloc.create().id;
        let b = alloc.create().id;
        let c = alloc.create().id;

        map.insert(a, 0);
        map.insert(b, 1);
        map.insert(c, 2);

        alloc.kill(b);

        map.retain_living(&alloc);

        assert_eq!(Some(&0), map.get_unchecked(a));
        assert_eq!(None, map.get_unchecked(b));
        assert_eq!(Some(&2), map.get_unchecked(c));
    }
}