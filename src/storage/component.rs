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

impl<A: Arena<Generation = ()>, T> Get<Id<A>, T> for Component<A, T> {
    fn get(&self, id: Id<A>) -> &T {
        self.get_unchecked(id)
    }

    fn get_mut(&mut self, id: Id<A>) -> &mut T {
        self.get_mut_unchecked(id)
    }
}

impl<A: Arena<Generation = ()>, T> Get<&Id<A>, T> for Component<A, T> {
    fn get(&self, id: &Id<A>) -> &T {
        self.get_unchecked(*id)
    }

    fn get_mut(&mut self, id: &Id<A>) -> &mut T {
        self.get_mut_unchecked(*id)
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, T> Get<Valid<'_, A>, T> for Component<A, T> {
    fn get(&self, id: Valid<A>) -> &T {
        self.get_unchecked(id.id)
    }

    fn get_mut(&mut self, id: Valid<A>) -> &mut T {
        self.get_mut_unchecked(id.id)
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, T> Get<&Valid<'_, A>, T> for Component<A, T> {
    fn get(&self, id: &Valid<A>) -> &T {
        self.get_unchecked(id.id)
    }

    fn get_mut(&mut self, id: &Valid<A>) -> &mut T {
        self.get_mut_unchecked(id.id)
    }
}

impl<A: Arena<Generation = ()>, T> Insert<Id<A>, T> for Component<A, T> {
    fn insert(&mut self, id: Id<A>, value: T) {
        self.insert_unchecked(id, value);
    }
}

impl<A: Arena<Generation = ()>, T> Insert<&Id<A>, T> for Component<A, T> {
    fn insert(&mut self, id: &Id<A>, value: T) {
        self.insert_unchecked(*id, value);
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, T> Insert<Valid<'_, A>, T> for Component<A, T> {
    fn insert(&mut self, id: Valid<A>, value: T) {
        self.insert_unchecked(id.id, value);
    }
}

impl<A: Arena<Generation = G>, G: Dynamic, T> Insert<&Valid<'_, A>, T> for Component<A, T> {
    fn insert(&mut self, id: &Valid<A>, value: T) {
        self.insert_unchecked(id.id, value);
    }
}

impl<A: Arena, T> Component<A, T> {
    pub fn iter(&self) -> impl Iterator<Item = &T> {
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
        let index = id.to_usize();

        debug_assert!(self.values.len() >= index);

        if let Some(component) = self.values.get_mut(index) {
            *component = value;
        } else if self.values.len() == index {
            self.values.push(value);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::FixedArena;

    #[test]
    #[should_panic]
    fn get_at_invalid_index_panics() {
        let components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.get(id);
    }

    #[test]
    fn get_at_valid_index() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.insert(id, 5);

        assert_eq!(&5, components.get(id));
    }

    #[test]
    fn insert_at_index_equals_len_extends_vec() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.insert(id, 5);

        assert_eq!(1, components.len());
        assert_eq!(5, components.values[0]);
    }

    #[test]
    fn insert_at_index_less_than_len_replaces_existing_value() {
        let mut components = Component::<FixedArena, u32>::default();
        let id = Id { index: 0, gen: () };

        components.insert(id, 3);
        components.insert(id, 5);

        assert_eq!(1, components.len());
        assert_eq!(5, components.values[0]);
    }
}