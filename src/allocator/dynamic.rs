use crate::*;
use bit_vec::BitVec;

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct DynamicAllocator<A> {
    current_gen: Vec<Id<A>>,
    dead: Vec<u32>,
    living: BitVec,
}

impl<A> DynamicAllocator<A> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            current_gen: Vec::with_capacity(capacity),
            dead: Vec::default(),
            living: BitVec::with_capacity(capacity),
        }
    }

    pub fn validate(&self, id: impl TryIndexes<A>) -> Option<Valid<A>> {
        let id = id.id()?;

        if self.is_alive(id) {
            Some(Valid::new(id))
        } else {
            None
        }
    }
}

impl<A> Default for DynamicAllocator<A> {
    fn default() -> Self {
        Self {
            current_gen: vec![],
            dead: vec![],
            living: Default::default(),
        }
    }
}

impl<A> DynamicAllocator<A> {
    pub fn create(&mut self) -> Valid<A> {
        let id = if let Some(index) = self.dead.pop() {
            self.reuse_index(index)
        } else {
            self.create_new()
        };

        Valid::new(id)
    }

    fn reuse_index(&mut self, index: u32) -> Id<A> {
        let i = index as usize;

        self.living.set(i, true);

        self.current_gen[i]
    }

    fn create_new(&mut self) -> Id<A> {
        let index = self.current_gen.len() as u32;

        let id = Id::first(index);

        self.current_gen.push(id);
        self.living.push(true);

        id
    }

    pub fn kill(&mut self, id: Id<A>) {
        if self.is_alive(id) {
            let index = id.get_u32();
            let i = index as usize;

            if let Some(current_id) = self.current_gen.get_mut(i) {
                *current_id = current_id.next_gen();
            }

            self.dead.push(index);
            self.living.set(i, false);
        }
    }

    pub fn is_alive(&self, id: Id<A>) -> bool {
        if let Some(current_id) = self.current_gen.get(id.get_index()) {
            id.eq(&current_id)
        } else {
            panic!("{}: Invalid id", std::any::type_name::<Self>())
        }
    }

    pub fn living(&self) -> impl Iterator<Item = bool> + '_ {
        self.living.iter()
    }

    pub fn ids(&self) -> impl Iterator<Item = Option<ValidRef<A>>> {
        self.current_gen.iter()
            .zip(self.living.iter())
            .map(|(id, live)| {
                if live {
                    Some(ValidRef::new(id))
                } else {
                    None
                }
            })
    }

    pub fn zip_id_and_filter<I: Iterator<Item=T>, T>(&self, iter: I) -> impl Iterator<Item=(T, ValidRef<A>)> {
        iter.zip(self.ids())
            .filter_map(|(t, id)| {
                id.map(|id| (t, id))
            })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::GenerationalArena;

    #[test]
    fn create_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        assert_eq!(
            Id::first(0),
            gen_allocator.create().id
        );
        assert_eq!(
            Id::first(1),
            gen_allocator.create().id
        );
    }

    #[test]
    fn reuse_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        let id1 = gen_allocator.create().id;
        gen_allocator.kill(id1);

        let reused_id = Id::first(0).next_gen();
        assert_eq!(
            reused_id,
            gen_allocator.create().id
        );
        assert_eq!(
            Id::first(1),
            gen_allocator.create().id
        );
    }

    #[test]
    fn validate_valid_returns_some() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id = allocator.create().id;

        assert!(allocator.validate(id).is_some());
    }

    #[test]
    fn validate_invalid_returns_none() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id = allocator.create().id;
        allocator.kill(id);

        assert!(allocator.validate(id).is_none());
    }

    #[test]
    fn gen_alloc_lifetime_test() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id1 = allocator.create().id; // Remove ".id" to cause a compiler error
        let id2 = allocator.create();

        println!("{:?}", id1);
        println!("{:?}", id2);
    }
}
