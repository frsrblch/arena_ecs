use crate::*;
use std::any::type_name;
use std::convert::TryFrom;
use bit_vec::BitVec;

pub trait Validate<ID, A: Arena> where A::Generation: Dynamic {
    fn validate(&self, id: ID) -> Option<Valid<A>>;
}

impl<A: Arena> Validate<Id<A>, A> for DynamicAllocator<A> where A::Generation: Dynamic {
    fn validate(&self, id: Id<A>) -> Option<Valid<'_, A>> {
        if self.is_alive(id) {
            Some(Valid::new(id))
        } else {
            None
        }
    }
}

impl<A: Arena> Validate<&Id<A>, A> for DynamicAllocator<A> where A::Generation: Dynamic {
    fn validate(&self, id: &Id<A>) -> Option<Valid<'_, A>> {
        if self.is_alive(*id) {
            Some(Valid::new(*id))
        } else {
            None
        }
    }
}

#[derive(Debug)]
pub struct DynamicAllocator<A: Arena>
    where A::Generation: Dynamic
{
    gen: Vec<A::Generation>,
    dead: Vec<A::Index>,
    living: BitVec,
}

impl<A: Arena> Default for DynamicAllocator<A> where A::Generation: Dynamic {
    fn default() -> Self {
        Self {
            gen: vec![],
            dead: vec![],
            living: Default::default(),
        }
    }
}

impl<A: Arena> DynamicAllocator<A> where A::Generation: Dynamic {
    pub fn create(&mut self) -> Valid<A> {
        let id = if let Some(index) = self.dead.pop() {
            self.reuse_index(index)
        } else {
            self.create_new()
        };

        Valid::new(id)
    }

    fn reuse_index(&mut self, index: A::Index) -> Id<A> {
        let i = index.index();
        let gen = self.gen[i];

        self.living.set(i, true);

        let new_id = Id { index, gen };

        new_id
    }

    fn create_new(&mut self) -> Id<A> {
        let i = self.gen.len();

        self.living.push(true);

        let index = <A::Index as TryFrom<usize>>::try_from(i)
            .ok()
            .expect(&format!("{}::create_new: usize out of range for index type: {}", type_name::<Self>(), type_name::<A::Index>()));

        let gen = A::Generation::first();
        self.gen.push(gen);

        Id { index, gen }
    }

    pub fn kill(&mut self, id: Id<A>) {
        if self.is_alive(id) {
            let i = id.index();

            if let Some(gen) = self.gen.get_mut(i) {
                *gen = gen.next_gen();
            }

            self.dead.push(id.index);
            self.living.set(i, false);
        }
    }

    pub fn is_alive(&self, id: Id<A>) -> bool {
        if let Some(gen) = self.gen.get(id.index()) {
            id.gen.eq(gen)
        } else {
            false
        }
    }

    pub fn living(&self) -> impl Iterator<Item=bool> + '_ {
        self.living.iter()
    }
}