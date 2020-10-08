use crate::*;
use std::iter::Zip;
use std::marker::PhantomData;

pub trait ValidId<A>: Copy {
    fn index(&self) -> usize;
    fn id(&self) -> Id<A>;
}

pub trait ValidEdge<A> {
    type Id: ValidId<A>;
    fn edge(&self) -> Edge<A>;
    fn from(&self) -> Self::Id;
    fn to(&self) -> Self::Id;
}

pub trait ArenaIterator: IntoIterator + Sized {
    type Arena: Arena;

    fn zip<U: ArenaIterator<Arena = Self::Arena>>(self, rhs: U) -> ArenaZip<Self::Arena, Self, U> {
        ArenaZip::new(self, rhs)
    }

    fn for_each<F: FnMut(Self::Item)>(self, f: F) {
        self.into_iter().for_each(f)
    }
}

pub struct ArenaZip<ID, A, B> {
    a: A,
    b: B,
    marker: std::marker::PhantomData<ID>,
}

impl<ID: Arena, A: ArenaIterator<Arena = ID>, B: ArenaIterator<Arena = ID>> ArenaZip<ID, A, B> {
    pub(crate) fn new(a: A, b: B) -> Self {
        Self {
            a,
            b,
            marker: PhantomData,
        }
    }
}

impl<ID: Arena, A: ArenaIterator<Arena = ID>, B: ArenaIterator<Arena = ID>> IntoIterator
    for ArenaZip<ID, A, B>
{
    type Item = (A::Item, B::Item);
    type IntoIter = Zip<A::IntoIter, B::IntoIter>;

    fn into_iter(self) -> Self::IntoIter {
        self.a.into_iter().zip(self.b.into_iter())
    }
}

impl<ID, A, B> ArenaIterator for ArenaZip<ID, A, B>
where
    ID: Arena,
    A: ArenaIterator<Arena = ID>,
    B: ArenaIterator<Arena = ID>,
{
    type Arena = ID;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::*;

    #[test]
    fn zip_test() {
        let mut a = Component::<FixedArena, u32>::default();
        let b = Component::<FixedArena, u32>::default();

        a.iter().zip(b.iter()).for_each(|(a, b)| {
            dbg!(a, b);
        });

        a.zip(&b).into_iter().for_each(|(a, b)| {
            dbg!(a, b);
        });

        for (a, b) in ArenaZip::new(&mut a, &b) {
            dbg!(a, b);
        }

        for (a, b) in a.zip_mut(&b).zip(&b) {
            dbg!(a, b);
        }

        // uncommenting breaks compilation
        // let c = Component::<GenerationalArena, u32>::default();
        // a.iter_mut().zip(c.iter());
        // a.zip(&c);
    }
}
