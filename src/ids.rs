use crate::*;

#[derive(Debug)]
pub struct Id<A: Arena>
{
    pub(crate) index: A::Index,
    pub(crate) gen: A::Generation,
}

impl<A: Arena> Id<A>
where A::Generation: Fixed
{
    pub(crate) fn index(self) -> usize {
        self.index.index()
    }
}

use std::hash::Hasher;
impl<A: Arena> Hash for Id<A> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.index.hash(state);
        self.gen.hash(state);
    }
}

impl<A: Arena> Clone for Id<A> {
    fn clone(&self) -> Self {
        Self {
            index: self.index,
            gen: self.gen
        }
    }
}

impl<A: Arena> Copy for Id<A> {}

impl<A: Arena> PartialEq for Id<A> {
    fn eq(&self, other: &Self) -> bool {
        self.index.eq(&other.index) && self.gen.eq(&other.gen)
    }
}

impl<A: Arena> Eq for Id<A> {}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, Clone)]
    struct Body;

    impl Arena for Body {
        type Index = u32;
        type Generation = ();
        type Dead = ();
    }

    #[test]
    fn id_test() {
        let id = Id::<Body> { index: 0, gen: () };

        assert_eq!(
            id,
            Id {
                index: 0,
                gen: ()
            }
        );
    }
}
