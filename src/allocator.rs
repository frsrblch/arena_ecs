use crate::*;
use std::any::type_name;

pub trait Create<'a, A: Arena, G: Fixed, ID> {
    fn create(&'a mut self) -> ID;
}

#[derive(Debug)]
pub struct Allocator<A: Arena> {
    gen: <A as Arena>::Generations,
    dead: <A as Arena>::Dead,
    next_index: <A as Arena>::NextIndex,
}

impl<A: Arena> Default for Allocator<A> {
    fn default() -> Self {
        Self {
            gen: A::Generations::default(),
            dead: A::Dead::default(),
            next_index: A::NextIndex::default(),
        }
    }
}

impl<'a, I, G, A> Create<'a, A, G, Valid<'a, A>> for Allocator<A>
where
    I: Index,
    G: Dynamic,
    A: Arena<Index=I, Generation=G, Generations=Vec<G>, Dead=Vec<I>>,
{
    fn create(&mut self) -> Valid<'a, A> {
        let id = if let Some(index) = self.dead.pop() {
            self.reuse_index(index)
        } else {
            self.create_new()
        };
        Valid::new(id)
    }
}

impl<I: Index + Default, A: Arena<Index=I, Generation=(), NextIndex=I>> Create<'_, A, (), Id<A>> for Allocator<A>
{
    fn create(&mut self) -> Id<A> {
        let index = self.next_index;
        self.next_index.increment();
        Id {
            index,
            gen: ()
        }
    }
}

impl<I: Index, G: Dynamic + Fixed, A: Arena<Index=I, Generation=G, Generations=Vec<G>, Dead=Vec<I>>> Allocator<A>
where
{
    fn reuse_index(&mut self, index: A::Index) -> Id<A> {
        let gen = *self.gen.get(index.index()).unwrap();
        Id { index, gen }
    }

    fn create_new(&mut self) -> Id<A> {
        let index = A::Index::try_from(self.gen.len())
            .ok()
            .expect(
                &format!(
                    "{}: usize out of range, could not convert to {}",
                    type_name::<Self>(),
                    type_name::<A::Index>()
                )
            );

        let gen = A::Generation::first();
        self.gen.push(gen);

        Id { index, gen }
    }

    pub fn kill(&mut self, id: Id<A>) {
        if let Some(gen) = self.gen.get_mut(id.index()) {
            if gen.eq(&&id.gen) {
                *gen = gen.next_gen();
                self.dead.push(id.index);
            }
        }

        if self.is_alive(id) {
            if let Some(gen) = self.gen.get_mut(id.index()) {
                *gen = gen.next_gen();
            }
        }
    }

    pub fn is_alive(&self, id: Id<A>) -> bool {
        self.gen
            .get(id.index())
            .map(|gen| gen.eq(&id.gen))
            .unwrap_or(false)
    }

    pub fn validate(&self, id: Id<A>) -> Option<Valid<A>> {
        if self.is_alive(id) {
            Some(Valid::new(id))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn vec_size() {
        assert_eq!(24, size_of::<Vec<()>>());
        assert_eq!(1, size_of::<Allocator<FixedArena>>());
        assert_eq!(48, size_of::<Allocator<GenerationalArena>>());
    }

    #[test]
    fn id_size() {
        assert_eq!(1, size_of::<Id<FixedArena>>());
        assert_eq!(2, size_of::<Id<GenerationalArena>>());
    }

    #[derive(Debug, Default)]
    struct FixedArena;

    fixed_arena!(FixedArena, u8);

    #[derive(Debug, Default)]
    struct GenerationalArena;

    dynamic_arena!(GenerationalArena, u8, NonZeroU8);

    #[test]
    #[should_panic]
    fn allocator_panic_when_index_out_of_range() {
        let mut allocator = Allocator::<FixedArena>::default();
        for _ in 0..257 {
            let _id = allocator.create();
        }
    }

    #[test]
    fn create_fixed() {
        let mut fixed_allocator = Allocator::<FixedArena>::default();

        assert_eq!(Id { index: 0, gen: () }, fixed_allocator.create());
        assert_eq!(Id { index: 1, gen: () }, fixed_allocator.create());
    }

    #[test]
    fn create_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        assert_eq!(Id { index: 0, gen: NonZeroU8::first() }, gen_allocator.create().id);
        assert_eq!(Id { index: 1, gen: NonZeroU8::first() }, gen_allocator.create().id);
    }

    #[test]
    fn reuse_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        let id1 = gen_allocator.create().id;
        gen_allocator.kill(id1);

        assert_eq!(Id { index: 0, gen: NonZeroU8::first().next_gen() }, gen_allocator.create().id);
        assert_eq!(Id { index: 1, gen: NonZeroU8::first() }, gen_allocator.create().id);
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