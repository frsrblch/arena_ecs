use crate::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct IdLink<A, B> {
    component: Component<A, Option<Id<B>>>,
    generation: u64,
}

impl<A, B> Default for IdLink<A, B> {
    fn default() -> Self {
        Self {
            component: Default::default(),
            generation: 0,
        }
    }
}

impl<A, B> std::ops::Deref for IdLink<A, B> {
    type Target = Component<A, Option<Id<B>>>;

    fn deref(&self) -> &Self::Target {
        &self.component
    }
}

impl<A, B> IdLink<A, B> {
    pub fn insert<IA: ValidId<A>, IB: ValidId<B>>(&mut self, id: IA, link: Option<IB>) {
        self.component.insert(id, link.map(|link| link.id()));
    }

    pub fn insert_unvalidated<I: ValidId<A>>(&mut self, id: I, link: Option<Id<B>>) {
        self.component.insert(id, link);

        if link.is_some() {
            self.generation = 0;
        }
    }

    pub fn remove<I: ValidId<A>>(&mut self, id: I) {
        *self.component.get_mut(id) = None;
    }
}

impl<A, B: Arena<Allocator = DynamicAllocator<B>>> IdLink<A, B> {
    pub fn kill(&mut self, id: Id<B>) {
        for link in self.component.iter_mut() {
            if *link == Some(id) {
                *link = None;
            }
        }

        self.generation += 1;
    }

    pub fn validate<'a>(&'a mut self, alloc: &'a Allocator<B>) -> Valid<'a, &Self> {
        if !self.is_synchronized(alloc) {
            self.retain_living(alloc);
        }

        Valid::new(self)
    }

    pub fn validate_mut<'a>(&'a mut self, alloc: &'a Allocator<B>) -> Valid<'a, &mut Self> {
        if !self.is_synchronized(alloc) {
            self.retain_living(alloc);
        }

        Valid::new(self)
    }

    pub fn try_validate<'a>(&'a self, alloc: &'a Allocator<B>) -> Option<Valid<'a, &Self>> {
        if self.is_synchronized(alloc) {
            Some(Valid::new(self))
        } else {
            None
        }
    }

    fn is_synchronized(&self, alloc: &Allocator<B>) -> bool {
        self.generation == alloc.generation()
    }

    fn retain_living(&mut self, alloc: &Allocator<B>) {
        for link in self.component.iter_mut() {
            if let Some(id) = link {
                if !alloc.is_alive(*id) {
                    *link = None;
                }
            }
        }

        self.generation = alloc.generation();
    }
}

impl<'a, A, B> Valid<'a, &IdLink<A, B>> {
    pub fn get<I: ValidId<A>>(&'a self, id: I) -> Option<Valid<'a, Id<B>>> {
        self.value.component.get(id).map(Valid::new)
    }

    pub fn iter(&'a self) -> Iter<A, B> {
        Iter {
            iter: self.value.component.iter().into_iter(),
            marker: PhantomData,
        }
    }
}

pub struct Iter<'a, A, B> {
    iter: std::slice::Iter<'a, Option<Id<B>>>,
    marker: PhantomData<A>,
}

impl<'a, A, B> Iterator for Iter<'a, A, B> {
    type Item = Option<Valid<'a, &'a Id<B>>>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(|id| id.as_ref().map(Valid::new))
    }
}

impl<'a, A, B> IntoIterator for &'a Valid<'a, &IdLink<A, B>> {
    type Item = Option<Valid<'a, &'a Id<B>>>;
    type IntoIter = Iter<'a, A, B>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, A: Arena, B> ArenaIterator for &'a Valid<'a, &IdLink<A, B>> {
    type Arena = A;
}
