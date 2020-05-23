use crate::*;
use std::any::type_name;
use std::convert::TryFrom;

#[derive(Debug)]
pub struct Allocator<A: Arena> {
    gen: <A::Generation as Fixed>::Vec,
    dead: <A as Arena>::Dead,
}

impl<A: Arena> Default for Allocator<A> {
    fn default() -> Self {
        Self {
            gen: <A::Generation as Fixed>::Vec::default(),
            dead: <A as Arena>::Dead::default(),
        }
    }
}

impl<A: Arena> Allocator<A>
where
    A::Generation: Generation,
    A::Dead: VecType<Item=A::Index>,
{
    pub fn create_or_reuse(&mut self) -> Id<A> {
        if let Some(index) = self.dead.pop() {
            let gen = *self.gen.get(index.index()).unwrap();
            Id { index, gen }
        } else {
            self.create_new()
        }
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
}

impl<A: Arena> Allocator<A> {
    pub fn create_new(&mut self) -> Id<A> {
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
}

#[cfg(test)]
mod test {
    use super::*;
    use std::mem::size_of;

    #[test]
    fn vec_size() {
        assert_eq!(24, size_of::<Vec<()>>());
        assert_eq!(8, size_of::<Allocator<FixedArena>>());
        assert_eq!(48, size_of::<Allocator<GenerationalArena>>());
    }

    #[test]
    fn id_size() {
        assert_eq!(1, size_of::<Id<FixedArena>>());
        assert_eq!(2, size_of::<Id<GenerationalArena>>());
    }

    #[test]
    fn debugging() {
        let mut fixed_allocator = Allocator::<FixedArena>::default();
        let id1 = fixed_allocator.create_new();

        let mut generational_allocator = Allocator::<GenerationalArena>::default();
        let id2 = generational_allocator.create_or_reuse();

        println!("fixed alloc: {:?}, id: {:?}", fixed_allocator, id1);
        println!("gen alloc: {:?}, id: {:?}", generational_allocator, id2);

        // panic!();
    }

    #[derive(Debug)]
    struct FixedArena;

    impl Arena for FixedArena {
        type Index = u8;
        type Generation = ();
        type Dead = ();
    }

    #[derive(Debug)]
    struct GenerationalArena;

    impl Arena for GenerationalArena {
        type Index = u8;
        type Generation = NonZeroU8;
    }

    #[test]
    #[should_panic]
    fn allocator_panic_when_index_out_of_range() {
        let mut allocator = Allocator::<FixedArena>::default();
        for _ in 0..257 {
            let _id = allocator.create_new();
        }
    }

    #[test]
    fn create_fixed() {
        let mut fixed_allocator = Allocator::<FixedArena>::default();

        assert_eq!(Id { index: 0, gen: () }, fixed_allocator.create_new());
        assert_eq!(Id { index: 1, gen: () }, fixed_allocator.create_new());
    }

    #[test]
    fn create_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        assert_eq!(Id { index: 0, gen: NonZeroU8::first() }, gen_allocator.create_new());
        assert_eq!(Id { index: 1, gen: NonZeroU8::first() }, gen_allocator.create_new());
    }

    #[test]
    fn reuse_generational() {
        let mut gen_allocator = Allocator::<GenerationalArena>::default();

        let id1 = gen_allocator.create_new();
        gen_allocator.kill(id1);

        assert_eq!(Id { index: 0, gen: NonZeroU8::first().next_gen() }, gen_allocator.create_or_reuse());
        assert_eq!(Id { index: 1, gen: NonZeroU8::first() }, gen_allocator.create_or_reuse());
    }
}