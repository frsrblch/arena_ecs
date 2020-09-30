use crate::*;
use fnv::{FnvHashMap as HashMap};


#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct IdMap<ID, T> {
    map: HashMap<Id<ID>, T>,
    generation: u64,
}

impl<ID, T: Clone> Clone for IdMap<ID, T> {
    fn clone(&self) -> Self {
        Self {
            map: self.map.clone(),
            generation: self.generation,
        }
    }
}

impl<ID, T> Default for IdMap<ID, T> {
    fn default() -> Self {
        Self {
            map: HashMap::default(),
            generation: 0,
        }
    }
}

impl<ID, T> IdMap<ID, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, Default::default()),
            generation: 0,
        }
    }

    /// Inserts a possibly-invalid Id and Value into the hashmap. Resets the IdMap's generation value.
    pub fn insert(&mut self, id: Id<ID>, value: T) {
        self.map.insert(id, value);
        self.generation = 0;
    }

    /// Inserts a valid Id and Value into the hashmap. Does not reset the IdMap's generation value.
    pub fn insert_valid<I: Indexes<ID>>(&mut self, id: I, value: T) {
        self.map.insert(id.id(), value);
    }

    pub fn get<I: TryIndexes<ID>>(&self, id: I) -> Option<&T> {
        let id = id.id()?;
        self.get_unchecked(id)
    }

    pub fn get_mut<I: TryIndexes<ID>>(&mut self, id: I) -> Option<&mut T> {
        let id = id.id()?;
        self.get_unchecked_mut(id)
    }

    fn get_unchecked(&self, id: Id<ID>) -> Option<&T> {
        self.map.get(&id)
    }

    fn get_unchecked_mut(&mut self, id: Id<ID>) -> Option<&mut T> {
        self.map.get_mut(&id)
    }

    pub fn remove(&mut self, id: Id<ID>) -> Option<T> {
        self.map.remove(&id)
    }

    pub fn kill(&mut self, id: Id<ID>) {
        self.remove(id);
        self.generation += 1;
    }

    pub fn retain<F: FnMut(&Id<ID>, &mut T) -> bool>(&mut self, f: F) {
        self.map.retain(f)
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }

    pub fn iter(&self) -> impl Iterator<Item=(&Id<ID>, &T)> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(&Id<ID>, &mut T)> {
        self.map.iter_mut()
    }

    pub fn values(&self) -> impl Iterator<Item=&T> {
        self.map.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item=&mut T> {
        self.map.values_mut()
    }
}

impl<ID: Arena<Allocator = DynamicAllocator<ID>>, T> IdMap<ID, T> {
    // pub fn validate<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> ValidMap<'a, ID, T> {
    //     if self.generation != allocator.generation() {
    //         self.retain_living(allocator);
    //     }
    //
    //     ValidMap { map: &mut (*self) }
    // }

    pub fn retain_living(&mut self, allocator: &Allocator<ID>) {
        self.map.retain(|id, _| allocator.is_alive(*id));
        self.generation = allocator.generation();
    }

    pub fn validate<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> Valid<&'a Self> {
        if self.generation != allocator.generation() {
            self.retain_living(allocator);
        }

        Valid::new(self)
    }

    pub fn validate_mut<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> Valid<&'a mut Self> {
        if self.generation != allocator.generation() {
            self.retain_living(allocator);
        }

        Valid::new(self)
    }

    pub fn try_validate<'a>(&'a self, allocator: &'a Allocator<ID>) -> Option<Valid<&'a Self>> {
        if self.generation == allocator.generation() {
            Some(Valid::new(self))
        } else {
            None
        }
    }
}

impl<'a, ID, T> Valid<'a, &'a IdMap<ID, T>> {
    pub fn iter(&'a self) -> impl Iterator<Item=(Valid<'a, &Id<ID>>, &T)> {
        self.value
            .map
            .iter()
            .map(|(id, value)| (Valid::new(id), value))
    }
}

impl<'a, ID, T> Valid<'a, &mut IdMap<ID, T>> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item=(Valid<'_, &Id<ID>>, &mut T)> {
        self.value
            .map
            .iter_mut()
            .map(|(id, value)| (Valid::new(id), value))
    }

    pub fn retain<F: FnMut(Valid<'a, Id<ID>>, &mut T) -> bool>(&mut self, mut f: F) {
        let f = |id: &Id<ID>, v: &mut T| f(Valid::new(*id), v);
        self.value.retain(f);
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

        let a = alloc.create().value;
        let b = alloc.create().value;
        let c = alloc.create().value;

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