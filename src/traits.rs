use crate::*;

pub trait ValidId<A>: Copy {
    fn index(self) -> usize;
    fn id(self) -> Id<A>;
}

pub trait ValidEdge<A> {
    type Id: ValidId<A>;
    fn edge(&self) -> Edge<A>;
    fn from(&self) -> Self::Id;
    fn to(&self) -> Self::Id;
}

#[cfg(test)]
mod test {
    use crate::allocator::test::GenerationalArena;
    use crate::*;

    #[derive(Debug)]
    struct Thing(Id<GenerationalArena>);

    impl Thing {
        pub fn new<'a>(id: impl ValidId<GenerationalArena> + 'a) -> Valid<'a, Self> {
            Valid::assert(Self(id.id()))
        }
    }

    #[test]
    fn valid_id_lifetime() {
        let mut a = Allocator::<GenerationalArena>::default();

        let valid = a.create();

        let _thing = Thing::new(valid);

        let _new_valid = a.create();

        // uncommenting should break compilation
        // dbg!(thing);
    }
}
