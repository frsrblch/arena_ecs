use crate::*;
use std::marker::PhantomData;

#[derive(Debug)]
pub struct Component<A: Arena, T> {
    values: Vec<T>,
    marker: PhantomData<A>,
}

impl<A: Arena, T> Default for Component<A, T> {
    fn default() -> Self {
        Self {
            values: vec![],
            marker: PhantomData,
        }
    }
}

impl<A: Arena<Generation=()>, T> Get<Id<A>, T> for Component<A, T> {
    fn get(&self, id: Id<A>) -> &T {
        self.get_unchecked(id)
    }

    fn get_mut(&mut self, id: Id<A>) -> &mut T {
        self.get_mut_unchecked(id)
    }
}

impl<A: Arena<Generation=()>, T> Get<&Id<A>, T> for Component<A, T> {
    fn get(&self, id: &Id<A>) -> &T {
        self.get_unchecked(*id)
    }

    fn get_mut(&mut self, id: &Id<A>) -> &mut T {
        self.get_mut_unchecked(*id)
    }
}

impl<A: Arena<Generation=G>, G: Dynamic, T> Get<Valid<'_, A>, T> for Component<A, T> {
    fn get(&self, id: Valid<A>) -> &T {
        self.get_unchecked(id.id)
    }

    fn get_mut(&mut self, id: Valid<A>) -> &mut T {
        self.get_mut_unchecked(id.id)
    }
}

impl<A: Arena<Generation=G>, G: Dynamic, T> Get<&Valid<'_, A>, T> for Component<A, T> {
    fn get(&self, id: &Valid<A>) -> &T {
        self.get_unchecked(id.id)
    }

    fn get_mut(&mut self, id: &Valid<A>) -> &mut T {
        self.get_mut_unchecked(id.id)
    }
}

impl<A: Arena<Generation=()>, T> Insert<Id<A>, T> for Component<A, T> {
    fn insert(&mut self, id: Id<A>, value: T) {
        self.insert_unchecked(id, value);
    }
}

impl<A: Arena<Generation=()>, T> Insert<&Id<A>, T> for Component<A, T> {
    fn insert(&mut self, id: &Id<A>, value: T) {
        self.insert_unchecked(*id, value);
    }
}

impl<A: Arena<Generation=G>, G: Dynamic, T> Insert<Valid<'_, A>, T> for Component<A, T> {
    fn insert(&mut self, id: Valid<A>, value: T) {
        self.insert_unchecked(id.id, value);
    }
}

impl<A: Arena<Generation=G>, G: Dynamic, T> Insert<&Valid<'_, A>, T> for Component<A, T> {
    fn insert(&mut self, id: &Valid<A>, value: T) {
        self.insert_unchecked(id.id, value);
    }
}

impl<A: Arena, T> Component<A, T> {
    pub fn iter(&self) -> impl Iterator<Item=&T> {
        self.values.iter()
    }

    pub fn len(&self) -> usize {
        self.values.len()
    }

    fn get_unchecked(&self, id: Id<A>) -> &T {
        self.values.get(id.to_usize()).expect(&format!(
            "{}: invalid id index ({:?})",
            std::any::type_name::<Self>(),
            id
        ))
    }

    fn get_mut_unchecked(&mut self, id: Id<A>) -> &mut T {
        self.values.get_mut(id.to_usize()).expect(&format!(
            "{}: invalid id index ({:?})",
            std::any::type_name::<Self>(),
            id
        ))
    }

    fn insert_unchecked(&mut self, id: Id<A>, value: T) {
        match self.values.len() {
            len if len > id.to_usize() => {
                self.values[id.to_usize()] = value;
            }
            len if len == id.to_usize() => {
                self.values.push(value);
            }
            _ => panic!(
                "{}: attempted to insert at invalid index ({:?})",
                std::any::type_name::<Self>(),
                id
            ),
        }
    }
}
