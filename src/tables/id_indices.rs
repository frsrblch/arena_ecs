use crate::ValidId;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct IdIndices<C, E> {
    values: Vec<Option<E>>,
    marker: PhantomData<C>,
}

impl<C, E> Default for IdIndices<C, E> {
    fn default() -> Self {
        Self {
            values: Vec::default(),
            marker: PhantomData,
        }
    }
}

impl<C, E> IdIndices<C, E> {
    pub fn insert<I: ValidId<C>, IE: Into<E>>(&mut self, id: I, index: IE) {
        self.insert_inner(id.index(), index.into());
    }

    fn insert_inner(&mut self, index: usize, value: E) {
        if let Some(v) = self.values.get_mut(index) {
            *v = Some(value);
        } else if self.values.len() == index {
            self.values.push(Some(value));
        } else {
            panic!("Invalid index: {:?}", std::any::type_name::<Self>());
        }
    }

    pub fn remove<I: ValidId<C>>(&mut self, id: I) -> Option<E> {
        self.remove_inner(id.index())
    }

    fn remove_inner(&mut self, index: usize) -> Option<E> {
        if let Some(value) = self.values.get_mut(index) {
            value.take()
        } else {
            None
        }
    }
}
