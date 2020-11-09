use crate::*;

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct IdMap<ID, T> {
    map: HashMap<Id<ID>, T>,
    generation: AllocGen<ID>,
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
            generation: AllocGen::default(),
        }
    }
}

impl<ID, T> IdMap<ID, T> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            map: HashMap::with_capacity_and_hasher(capacity, Default::default()),
            generation: AllocGen::default(),
        }
    }

    /// Inserts a valid Id and Value into the hashmap. Does not reset the IdMap's generation value.
    pub fn insert<I: ValidId<ID>>(&mut self, id: I, value: T) {
        self.map.insert(id.id(), value);
    }

    pub fn get<I: ValidId<ID>>(&self, id: I) -> Option<&T> {
        let id = id.id();
        self.get_unchecked(id)
    }

    pub fn get_mut<I: ValidId<ID>>(&mut self, id: I) -> Option<&mut T> {
        let id = id.id();
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
        self.generation.increment();
    }

    pub fn retain<F: FnMut(&Id<ID>, &mut T) -> bool>(&mut self, f: F) {
        self.map.retain(f)
    }

    pub fn clear(&mut self) {
        self.map.clear()
    }

    pub fn iter(&self) -> impl Iterator<Item = (&Id<ID>, &T)> {
        self.map.iter()
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&Id<ID>, &mut T)> {
        self.map.iter_mut()
    }

    pub fn values(&self) -> impl Iterator<Item = &T> {
        self.map.values()
    }

    pub fn values_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.map.values_mut()
    }
}

impl<ID: Arena<Allocator = DynamicAllocator<ID>>, T> IdMap<ID, T> {
    pub fn validate<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> Valid<&'a Self> {
        self.synchronize(allocator);

        Valid::new(self)
    }

    pub fn validate_mut<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> Valid<&'a mut Self> {
        self.synchronize(allocator);

        Valid::new(self)
    }

    fn synchronize(&mut self, allocator: &Allocator<ID>) {
        match allocator.generation_cmp(self.generation) {
            GenerationCmp::Valid => {}
            GenerationCmp::OffByOne(killed) => {
                self.remove(killed);
                self.generation = allocator.generation();
            }
            GenerationCmp::Outdated => {
                self.map.retain(|id, _| allocator.is_alive(*id));
                self.generation = allocator.generation();
            }
        }
    }
}

impl<'a, ID, T> Valid<'a, &'a IdMap<ID, T>> {
    pub fn iter(&'a self) -> impl Iterator<Item = (Valid<'a, &Id<ID>>, &T)> {
        self.value
            .map
            .iter()
            .map(|(id, value)| (Valid::new(id), value))
    }

    pub fn get(&self, id: Id<ID>) -> Option<&T> {
        self.value.get_unchecked(id)
    }
}

impl<'a, ID, T> Valid<'a, &mut IdMap<ID, T>> {
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (Valid<'_, &Id<ID>>, &mut T)> {
        self.value
            .map
            .iter_mut()
            .map(|(id, value)| (Valid::new(id), value))
    }

    pub fn get_mut(&mut self, id: Id<ID>) -> Option<&mut T> {
        self.value.get_unchecked_mut(id)
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
        let alloc = &mut Allocator::<Test>::default();
        let mut map = IdMap::<Test, u32>::default();

        let mut create = |value: u32| {
            let id = alloc.create();
            map.insert(id, value);
            id.value
        };

        let a = create(0);
        let b = create(1);
        let c = create(2);

        alloc.kill(b);

        let map = map.validate(alloc);

        assert_eq!(Some(&0), map.get(a));
        assert_eq!(None, map.get(b));
        assert_eq!(Some(&2), map.get(c));
    }
}
