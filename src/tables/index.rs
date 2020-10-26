use std::marker::PhantomData;

#[derive(Debug)]
pub struct Index<C>(u32, PhantomData<C>);

impl<C> Clone for Index<C> {
    fn clone(&self) -> Self {
        Index(self.0, PhantomData)
    }
}

impl<C> Copy for Index<C> {}

impl<C> PartialEq for Index<C> {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<C> Eq for Index<C> {}

impl<C> PartialOrd for Index<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<C> Ord for Index<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0)
    }
}

impl<C> std::hash::Hash for Index<C> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.hash(state);
    }
}

impl<C> Index<C> {
    pub(super) fn new(index: usize) -> Self {
        Index(index as u32, PhantomData)
    }

    pub(super) fn index(&self) -> usize {
        self.0 as usize
    }
}
