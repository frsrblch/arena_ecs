use crate::*;
use bit_vec::BitVec;
use std::fmt::Debug;

pub trait Validate<ID, A: Arena>
where
    A::Generation: Dynamic,
{
    fn validate(&self, id: ID) -> Option<Valid<A>>;
}

impl<A: Arena<Generation = G>, G: Dynamic> Validate<Id<A>, A> for DynamicAllocator<A> {
    fn validate(&self, id: Id<A>) -> Option<Valid<'_, A>> {
        if self.is_alive(id) {
            Some(Valid::new(id))
        } else {
            None
        }
    }
}

impl<A: Arena<Generation = G>, G: Dynamic> Validate<&Id<A>, A> for DynamicAllocator<A> {
    fn validate(&self, id: &Id<A>) -> Option<Valid<'_, A>> {
        self.validate(*id)
    }
}

impl<A: Arena<Generation = G>, G: Dynamic> Validate<Option<Id<A>>, A> for DynamicAllocator<A> {
    fn validate(&self, id: Option<Id<A>>) -> Option<Valid<'_, A>> {
        id.and_then(|id| self.validate(id))
    }
}

impl<A: Arena<Generation = G>, G: Dynamic> Validate<&Option<Id<A>>, A> for DynamicAllocator<A> {
    fn validate(&self, id: &Option<Id<A>>) -> Option<Valid<'_, A>> {
        id.and_then(|id| self.validate(id))
    }
}

#[derive(Debug)]
pub struct DynamicAllocator<A: Arena>
where
    A::Generation: Dynamic,
{
    current_gen: Vec<Id<A>>,
    dead: Vec<A::Index>,
    living: BitVec,
}

impl<A: Arena<Generation = G>, G: Dynamic> Default for DynamicAllocator<A> {
    fn default() -> Self {
        Self {
            current_gen: vec![],
            dead: vec![],
            living: Default::default(),
        }
    }
}

impl<A: Arena<Generation = G>, G: Dynamic> DynamicAllocator<A> {
    pub fn create(&mut self) -> Valid<A> {
        let id = if let Some(index) = self.dead.pop() {
            self.reuse_index(index)
        } else {
            self.create_new()
        };

        Valid::new(id)
    }

    fn reuse_index(&mut self, index: A::Index) -> Id<A> {
        let i = index.to_usize();

        self.living.set(i, true);

        self.current_gen[i]
    }

    fn create_new(&mut self) -> Id<A> {
        let index = self.current_gen.len();
        let index = A::Index::from_usize(index);

        let gen = A::Generation::first_gen();

        let id = Id { index, gen };
        self.current_gen.push(id);

        self.living.push(true);

        id
    }

    pub fn kill(&mut self, id: Id<A>) {
        if self.is_alive(id) {
            let i = id.index.to_usize();

            if let Some(id) = self.current_gen.get_mut(i) {
                id.gen = id.gen.next_gen();
            }

            self.dead.push(id.index);
            self.living.set(i, false);
        }
    }

    pub fn is_alive(&self, id: Id<A>) -> bool {
        if let Some(alloc_id) = self.current_gen.get(id.index.to_usize()) {
            id.gen.eq(&alloc_id.gen)
        } else {
            false
        }
    }

    pub fn living(&self) -> impl Iterator<Item = bool> + '_ {
        self.living.iter()
    }

    pub fn ids<'a>(&'a self) -> impl Iterator<Item = ValidRef<'a, A>> + 'a {
        self.current_gen.iter()
            .map(ValidRef::new)
            .zip(self.living.iter())
            .filter_map(|(id, live)| {
                if live {
                    Some(id)
                } else {
                    None
                }
            })
    }

    pub fn zip_id_and_filter<I: Iterator<Item=T>, T>(&self, iter: I) -> impl Iterator<Item=(T, Valid<A>)> {
        let valid_ids = self.current_gen
            .iter()
            .copied()
            .map(Valid::new);

        iter.zip(valid_ids)
            .zip(self.living.iter())
            .filter_map(|((t, id), live)| {
                if live {
                    Some((t, id))
                } else {
                    None
                }
            })
    }
}

impl<'a, A: Arena<Generation=G>, G: Dynamic> Validates<'a, A> for &'a DynamicAllocator<A> {
    type Id = Valid<'a, A>;

    fn validate(&self, id: Id<A>) -> Option<Self::Id> {
        if self.is_alive(id) {
            Some(Valid::new(id))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::GenerationalArena;

    #[test]
    #[should_panic]
    fn allocator_panic_when_index_out_of_range() {
        let mut allocator = Allocator::<GenerationalArena>::default();
        for _ in 0..257 {
            let _id = allocator.create();
        }
    }

    #[test]
    fn create_fixed() {
        let mut fixed_allocator = Allocator::<GenerationalArena>::default();

        assert_eq!(
            Id {
                index: 0,
                gen: NonZeroU8::first_gen()
            },
            fixed_allocator.create().id
        );
        assert_eq!(
            Id {
                index: 1,
                gen: NonZeroU8::first_gen()
            },
            fixed_allocator.create().id
        );
    }

    #[test]
    fn create_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        assert_eq!(
            Id {
                index: 0,
                gen: NonZeroU8::first_gen()
            },
            gen_allocator.create().id
        );
        assert_eq!(
            Id {
                index: 1,
                gen: NonZeroU8::first_gen()
            },
            gen_allocator.create().id
        );
    }

    #[test]
    fn reuse_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        let id1 = gen_allocator.create().id;
        gen_allocator.kill(id1);

        assert_eq!(
            Id {
                index: 0,
                gen: NonZeroU8::first_gen().next_gen()
            },
            gen_allocator.create().id
        );
        assert_eq!(
            Id {
                index: 1,
                gen: NonZeroU8::first_gen()
            },
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
