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

    /// Inserts a possibly-invalid Id and Value into the hashmap. Invalidates the IdMap's generation value.
    pub fn insert(&mut self, id: Id<ID>, value: T) {
        self.map.insert(id, value);
        self.generation = 0;
    }

    /// Inserts a valid Id and Value into the hashmap. Does not invalidate the IdMap's generation value.
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
    pub fn validate<'a>(&'a mut self, allocator: &'a Allocator<ID>) -> ValidMap<'a, ID, T> {
        if self.generation != allocator.generation() {
            self.retain_living(allocator);
        }

        ValidMap { map: &mut (*self) }
    }

    pub fn retain_living(&mut self, allocator: &Allocator<ID>) {
        self.map.retain(|id, _| allocator.is_alive(*id));
        self.generation = allocator.generation();
    }
}

#[derive(Debug)]
pub struct ValidMap<'a, ID, T> {
    map: &'a mut IdMap<ID, T>,
}

impl<'a, ID, T> ValidMap<'a, ID, T> {
    pub fn iter(&self) -> impl Iterator<Item=(ValidRef<ID>, &T)> {
        self.map
            .map
            .iter()
            .map(|(id, value)| (ValidRef::new(id), value))
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item=(ValidRef<ID>, &mut T)> {
        self.map
            .map
            .iter_mut()
            .map(|(id, value)| (ValidRef::new(id), value))
    }


    pub fn retain<F: FnMut(ValidRef<ID>, &mut T) -> bool>(&mut self, mut f: F) {
        let f = |id: &Id<ID>, v: &mut T| f(ValidRef::new(id), v);
        self.map.retain(f)
    }
}

use std::ops::AddAssign;
impl<'a, ID, T: AddAssign<U>, U: Copy> AddAssign<ValidMap<'a, ID, U>> for Component<ID, T> {
    fn add_assign(&mut self, map: ValidMap<ID, U>) {
        *self += map
            .iter()
            .map(|(id, value)| (id, *value));
    }
}

impl<'a, ID, T, U> AddAssign<&'a IdMap<ID, U>> for Component<ID, T>
    where
        ID: Arena<Allocator=FixedAllocator<ID>>,
        T: AddAssign<U>,
        U: Copy,
{
    fn add_assign(&mut self, map: &'a IdMap<ID, U>) {
        *self += map
            .iter()
            .map(|(id, value)| (id, *value));
    }
}

impl<ID, I: Indexes<ID>, T: AddAssign<U>, U, ITER: Iterator<Item=(I, U)>> AddAssign<ITER> for Component<ID, T> {
    fn add_assign(&mut self, rhs: ITER) {
        rhs.for_each(|(id, value)| {
            let component_value = self.get_mut(id);
            *component_value += value;
        })
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