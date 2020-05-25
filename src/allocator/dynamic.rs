use crate::*;
use std::any::type_name;
use std::convert::TryFrom;
use bit_vec::BitVec;

pub trait Validate<ID, A: Arena> where A::Generation: Dynamic {
    fn validate(&self, id: ID) -> Option<Valid<A>>;
}

impl<A: Arena<Generation=G>, G: Dynamic> Validate<Id<A>, A> for DynamicAllocator<A> {
    fn validate(&self, id: Id<A>) -> Option<Valid<'_, A>> {
        if self.is_alive(id) {
            Some(Valid::new(id))
        } else {
            None
        }
    }
}

impl<A: Arena<Generation=G>, G: Dynamic> Validate<&Id<A>, A> for DynamicAllocator<A> {
    fn validate(&self, id: &Id<A>) -> Option<Valid<'_, A>> {
        self.validate(*id)
    }
}

#[derive(Debug)]
pub struct DynamicAllocator<A: Arena>
    where A::Generation: Dynamic
{
    current_gen: Vec<A::Generation>,
    dead: Vec<A::Index>,
    living: BitVec,
}

impl<A: Arena<Generation=G>, G: Dynamic> Default for DynamicAllocator<A> {
    fn default() -> Self {
        Self {
            current_gen: vec![],
            dead: vec![],
            living: Default::default(),
        }
    }
}

impl<A: Arena<Generation=G>, G: Dynamic> DynamicAllocator<A> {
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

        let gen = self.current_gen[i];

        self.living.set(i, true);

        Id { index, gen }
    }

    fn create_new(&mut self) -> Id<A> {
        let index = self.current_gen.len();

        let gen = A::Generation::first();
        self.current_gen.push(gen);

        self.living.push(true);

        let index = Self::get_index(index);

        Id { index, gen }
    }

    fn get_index(i: usize) -> A::Index {
        <A::Index as TryFrom<usize>>::try_from(i)
            .ok()
            .expect(&format!("{}::create_new: usize out of range for index type: {}", type_name::<Self>(), type_name::<A::Index>()))
    }

    pub fn kill(&mut self, id: Id<A>) {
        if self.is_alive(id) {
            let i = id.index();

            if let Some(gen) = self.current_gen.get_mut(i) {
                *gen = gen.next_gen();
            }

            self.dead.push(id.index);
            self.living.set(i, false);
        }
    }

    pub fn is_alive(&self, id: Id<A>) -> bool {
        if let Some(gen) = self.current_gen.get(id.index()) {
            id.gen.eq(gen)
        } else {
            false
        }
    }

    pub fn living(&self) -> impl Iterator<Item=bool> + '_ {
        self.living.iter()
    }
}
