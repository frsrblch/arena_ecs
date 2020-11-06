use crate::allocator::alloc_gen::GenerationCmp;
use crate::*;
use bit_vec::BitVec;
use std::iter::Zip;
use std::marker::PhantomData;

// #[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Debug)]
pub struct DynamicAllocator<ARENA> {
    current_gen: Vec<Id<ARENA>>,
    dead: Vec<u32>,
    living: BitVec,
    generation: AllocGen<ARENA>,
    last_killed: Option<Id<ARENA>>,
}

impl<ARENA> DynamicAllocator<ARENA> {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            current_gen: Vec::with_capacity(capacity),
            dead: Vec::default(),
            living: BitVec::with_capacity(capacity),
            generation: AllocGen::default(),
            last_killed: None,
        }
    }

    pub fn validate(&self, id: Id<ARENA>) -> Option<Valid<Id<ARENA>>> {
        if self.is_alive(id) {
            Some(Valid::new(id))
        } else {
            None
        }
    }

    pub fn generation(&self) -> AllocGen<ARENA> {
        self.generation
    }
}

impl<ARENA> Default for DynamicAllocator<ARENA> {
    fn default() -> Self {
        Self {
            current_gen: vec![],
            dead: vec![],
            living: Default::default(),
            generation: Default::default(),
            last_killed: None,
        }
    }
}

impl<ARENA> DynamicAllocator<ARENA> {
    pub fn create(&mut self) -> Valid<Id<ARENA>> {
        let id = if let Some(index) = self.dead.pop() {
            self.reuse_index(index)
        } else {
            self.create_new()
        };

        Valid::new(id)
    }

    fn reuse_index(&mut self, index: u32) -> Id<ARENA> {
        let i = index as usize;

        self.living.set(i, true);

        self.current_gen[i]
    }

    fn create_new(&mut self) -> Id<ARENA> {
        let index = self.current_gen.len() as u32;

        let id = Id::first(index);

        self.current_gen.push(id);
        self.living.push(true);

        id
    }

    pub fn kill(&mut self, id: Id<ARENA>) -> bool {
        if self.is_alive(id) {
            self.kill_unchecked(id);
            true
        } else {
            false
        }
    }

    fn kill_unchecked(&mut self, id: Id<ARENA>) {
        let index = id.get_u32();
        let i = index as usize;

        if let Some(current_id) = self.current_gen.get_mut(i) {
            *current_id = current_id.next_gen();
        }

        self.dead.push(index);
        self.living.set(i, false);
        self.generation.increment();
        self.last_killed = Some(id);
    }

    pub fn is_alive(&self, id: Id<ARENA>) -> bool {
        if let Some(current_id) = self.current_gen.get(id.get_index()) {
            id.eq(&current_id)
        } else {
            false
        }
    }

    pub fn last_killed(&self) -> Option<Id<ARENA>> {
        self.last_killed
    }

    pub fn living(&self) -> Living<ARENA> {
        Living::new(self)
    }

    pub fn ids(&self) -> Ids<ARENA> {
        Ids::new(self)
    }

    pub fn generation_cmp(&self, gen: AllocGen<ARENA>) -> GenerationCmp<ARENA> {
        match self.generation - gen {
            0 => GenerationCmp::Valid,
            // UNWRAP: an Id must have been killed for there to be a difference in generation
            1 => GenerationCmp::OffByOne(self.last_killed.unwrap()),
            _ => GenerationCmp::Outdated,
        }
    }
}

impl<ID: Arena<Allocator = DynamicAllocator<ID>>> DynamicAllocator<ID> {
    pub fn zip_id_and_filter<
        'a,
        I: TypedIterator<Context = ID> + IntoIterator<Item = T> + 'a,
        T,
    >(
        &self,
        iter: I,
    ) -> impl Iterator<Item = (T, Valid<&Id<ID>>)> {
        iter.zip(self.ids())
            .into_iter()
            .filter_map(|(t, id)| id.map(|id| (t, id)))
    }

    pub fn filter_living<'a, I: TypedIterator<Context = ID> + IntoIterator<Item = T> + 'a, T>(
        &'a self,
        iter: I,
    ) -> impl Iterator<Item = T> + 'a {
        iter.zip(self.living())
            .into_iter()
            .filter_map(|(t, alive)| if alive { Some(t) } else { None })
    }
}

pub struct Living<'a, ID> {
    bits: bit_vec::Iter<'a>,
    marker: PhantomData<ID>,
}

impl<'a, ID> Living<'a, ID> {
    fn new(alloc: &'a DynamicAllocator<ID>) -> Self {
        Self {
            bits: alloc.living.iter(),
            marker: PhantomData,
        }
    }
}

impl<'a, ID> IntoIterator for Living<'a, ID> {
    type Item = bool;
    type IntoIter = bit_vec::Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        self.bits
    }
}

impl<'a, ID> TypedIterator for Living<'a, ID> {
    type Context = ID;
}

pub struct Ids<'a, ID> {
    iter: Zip<std::slice::Iter<'a, Id<ID>>, bit_vec::Iter<'a>>,
}

impl<'a, ID> Ids<'a, ID> {
    fn new(alloc: &'a DynamicAllocator<ID>) -> Self {
        Self {
            iter: alloc.current_gen.iter().zip(alloc.living.iter()),
        }
    }
}

impl<'a, ID> Iterator for Ids<'a, ID> {
    type Item = Option<Valid<'a, &'a Id<ID>>>;
    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(
            |(id, living)| {
                if living {
                    Some(Valid::new(id))
                } else {
                    None
                }
            },
        )
    }
}

impl<'a, ID> TypedIterator for Ids<'a, ID> {
    type Context = ID;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::allocator::test::GenerationalArena;

    #[test]
    fn create_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        assert_eq!(Id::first(0), gen_allocator.create().value);
        assert_eq!(Id::first(1), gen_allocator.create().value);
    }

    #[test]
    fn reuse_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        let id1 = gen_allocator.create().value;
        gen_allocator.kill(id1);

        let reused_id = Id::first(0).next_gen();
        assert_eq!(reused_id, gen_allocator.create().value);
        assert_eq!(Id::first(1), gen_allocator.create().value);
    }

    #[test]
    fn validate_valid_returns_some() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id = allocator.create().value;

        assert!(allocator.validate(id).is_some());
    }

    #[test]
    fn validate_invalid_returns_none() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id = allocator.create().value;
        allocator.kill(id);

        assert!(allocator.validate(id).is_none());
    }

    #[test]
    fn gen_alloc_lifetime_test() {
        let mut allocator = Allocator::<GenerationalArena>::default();

        let id1 = allocator.create().value; // Remove ".id" to cause a compiler error
        let id2 = allocator.create();

        println!("{:?}", id1);
        println!("{:?}", id2);
    }
}
